#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <numeric>
#include <random>
#include <stdexcept>
#include <string>
#include <vector>
#include <omp.h>

using Time = std::chrono::steady_clock;
using Sum = std::uint64_t;
double elapsed(Time::time_point start) {
    return std::chrono::duration<double>(Time::now() - start).count();
}

struct Counts { Sum comparisons=0, shifts=0; };
inline void insert(std::uint32_t* a, int i, int h, Counts& counts) {
    const auto x=a[i];
    int j=i;
    while (j>=h) {
        ++counts.comparisons;
        if (a[j-h] <= x) break;
        a[j]=a[j-h];
        j-=h;
        ++counts.shifts;
    }
    if (j!=i) a[j]=x;
}
Counts layer(std::vector<std::uint32_t>& a, int h) {
    const int n=static_cast<int>(a.size());
    Sum comps=0, shifts=0;
    if (h<256) {
        Counts counts;
        for (int i=h; i<n; ++i) insert(a.data(), i, h, counts);
        return counts;
    }
    // Each worker owns distinct residue classes. Visit adjacent residues in tiles.
    const int tile=std::min(2048, std::max(16,h/48));
    const int tiles=(h+tile-1)/tile;
    #pragma omp parallel for schedule(static) reduction(+:comps,shifts)
    for (int b=0; b<tiles; ++b) {
        Counts counts;
        const int lo=b*tile, hi=std::min(h,lo+tile);
        for (std::int64_t row=h; row+lo<n; row+=h) {
            const int last=static_cast<int>(std::min<std::int64_t>(n,row+hi));
            for (int i=static_cast<int>(row+lo); i<last; ++i) insert(a.data(),i,h,counts);
        }
        comps+=counts.comparisons;
        shifts+=counts.shifts;
    }
    return {comps,shifts};
}
int main(int argc,char** argv) {
    if (argc!=5 && argc!=7) return 2;
    int n=std::stoi(argv[1]);
    if(n<2 || n>1000000000) return 2;
    const auto seed=std::stoull(argv[2]);
    std::ifstream table(argv[3]);
    std::vector<int> gaps;
    int h;
    while(table>>h) gaps.push_back(h);
    if(gaps.empty() || gaps.back()!=1) return 2;
    int prev=n;
    for(int gap:gaps) { if(gap<=0 || gap>=prev) return 2; prev=gap; }
    const int stop_at=argc==7 ? std::stoi(argv[5]) : 0;
    const int window_length=argc==7 ? std::stoi(argv[6]) : 131072;
    if (argc==7 && (n!=1000000000 || std::find(gaps.begin(),gaps.end(),stop_at)==gaps.end() ||
                    window_length<=0 || window_length>n/12)) return 2;
    omp_set_num_threads(std::stoi(argv[4]));
    std::vector<std::uint32_t> keys(n);
    auto start=Time::now();
    std::iota(keys.begin(),keys.end(),0u);
    std::mt19937_64 rng(seed);
    std::shuffle(keys.begin(),keys.end(),rng);
    std::cout<<"n "<<n<<" seed "<<seed<<" shuffle_s "<<elapsed(start)<<std::endl;
    auto ref=n<=200000 ? keys : std::vector<std::uint32_t>{};
    Counts total;
    for(int gap:gaps) {
        auto before=Time::now();
        auto c=layer(keys,gap);
        if(!ref.empty()) {
            Counts check;
            for(int i=gap;i<n;++i) {
                int j=i;
                while(j>=gap) {
                    ++check.comparisons;
                    if(ref[j-gap]<=ref[j]) break;
                    std::swap(ref[j-gap],ref[j]); j-=gap; ++check.shifts;
                }
            }
            if(check.comparisons!=c.comparisons || check.shifts!=c.shifts || keys!=ref) return 3;
        }
        total.comparisons+=c.comparisons; total.shifts+=c.shifts;
        std::cout<<"gap "<<gap<<" comparisons "<<c.comparisons<<" shifts "<<c.shifts<<" sec "<<elapsed(before)<<std::endl;
        if(n==1000000000 && (stop_at ? gap==stop_at : (gap==644 || gap==301))) {
            const std::string name="state_"+std::to_string(seed)+"_"+std::to_string(gap);
            const int len=window_length;
            std::ofstream data(name+".bin",std::ios::binary);
            std::ofstream meta(name+".csv");
            meta<<"offset,length\n";
            for(int j=0;j<12;++j) {
                const int offset=static_cast<int>(1LL*j*(n-len)/11);
                data.write(reinterpret_cast<const char*>(keys.data()+offset),len*sizeof(keys[0]));
                meta<<offset<<","<<len<<"\n";
            }
            if(!data || !meta) return 5;
            int worst=0;
            for(int j=0;j<n;++j) worst=std::max(worst,std::abs(static_cast<int>(keys[j])-j));
            std::cout<<"state "<<name<<" global_max_displacement "<<worst<<std::endl;
            if(stop_at) {
                std::cout<<"checkpoint_total "<<total.comparisons<<" shifts "<<total.shifts<<" elapsed_s "<<elapsed(start)<<std::endl;
                return 0;
            }
        }
    }
    for(int i=0;i<n;++i) if(keys[i]!=static_cast<std::uint32_t>(i)) return 4;
    std::cout<<"verified_total "<<total.comparisons<<" shifts "<<total.shifts<<" elapsed_s "<<elapsed(start)<<std::endl;
}
