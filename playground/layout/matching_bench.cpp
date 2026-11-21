// Compare traversal orders for the same number of fixed disjoint compare-exchanges.
// Timings cover the scope shown in each CSV row; machine setup affects elapsed time.
#include <algorithm>
#include <array>
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <malloc.h>
#include <memory>
#include <numeric>
#include <random>
#include <stdexcept>
#include <string>
#include <vector>
#define NOMINMAX
#include <windows.h>

using Key = std::uint64_t;
using Schedule = std::vector<std::uint32_t>;
using Clock = std::chrono::steady_clock;

struct AlignedFree {
    void operator()(Key* p) const { _aligned_free(p); }
};
using Array = std::unique_ptr<Key[], AlignedFree>;
Array array_of(size_t n) {
    Array p(static_cast<Key*>(_aligned_malloc(sizeof(Key) * n, 64)));
    if (!p) throw std::bad_alloc();
    return p;
}

// All candidates pay for the same indirect lookup and two assignments.
__declspec(noinline) void layer(Key* a, const std::uint32_t* order, size_t k) {
    for (size_t i = 0; i < k; ++i) {
        Key& right = a[k + order[i]];
        const Key x = a[i], y = right;
        a[i] = std::min(x, y);
        right = std::max(x, y);
    }
}

__declspec(noinline) void insert_tail(Key* a, size_t n) {
    for (size_t i = 1; i < n; ++i) {
        const Key x = a[i];
        size_t j = i;
        while (j > 0 && x < a[j - 1]) {
            a[j] = a[j - 1];
            --j;
        }
        a[j] = x;
    }
}

void scramble(Schedule& order) {
    std::uint32_t state = 5819;
    for (size_t size = order.size(); size > 1; --size) {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        std::swap(order[size - 1], order[state % size]);
    }
}

std::array<Schedule, 6> schedules(size_t k, bool hierarchical) {
    std::array<Schedule, 6> out;
    for (size_t slot = 0; slot < (hierarchical ? 6u : 5u); ++slot) {
        out[slot].resize(k);
        std::iota(out[slot].begin(), out[slot].end(), std::uint32_t{0});
    }
    std::reverse(out[1].begin(), out[1].end());
    // A fixed reproducible scrambled table, also used in the trace simulator.
    scramble(out[2]);
    const size_t lines = k / 8;
    Schedule line_order(lines);
    std::iota(line_order.begin(), line_order.end(), std::uint32_t{0});
    scramble(line_order);
    for (size_t i = 0; i < k; ++i) {
        out[3][i] = static_cast<std::uint32_t>(8 * (i % lines) + i / lines);
        out[4][i] = 8 * line_order[i / 8] + static_cast<std::uint32_t>(i % 8);
    }
    std::uint32_t node_seed = 7523;
    size_t pos = 0;
    auto visit = [&](auto&& self, size_t a, size_t count) -> void {
        if (count == 8) {
            for (size_t j = 0; j < 8; ++j) out[5][pos++] = static_cast<std::uint32_t>(a+j);
            return;
        }
        node_seed ^= node_seed << 13;
        node_seed ^= node_seed >> 17;
        node_seed ^= node_seed << 5;
        const auto h = count/2;
        const bool flip = (node_seed & 1) != 0;
        self(self, a + (flip ? h : 0), h);
        self(self, a + (flip ? 0 : h), h);
    };
    if (hierarchical) visit(visit, 0, k);
    return out;
}

void run(size_t n, size_t rows, bool tail, bool hierarchical = false) {
    const size_t k = n / 2;
    const auto order = schedules(k, hierarchical);
    auto input = array_of(n * rows);
    auto data = array_of(n * rows);
    const std::array<const char*, 6> names = {"forward", "reverse", "shuffled", "striped", "line_shuffled", "dyadic"};
    const int count = hierarchical ? 3 : 5;
    const std::array<int, 3> selected = {0, 4, 5};
    const int repeats = hierarchical ? 12 : 10;
    for (int trial = -2; trial < repeats; ++trial) {
        const auto seed = (hierarchical ? 771129u : 654321u) + static_cast<unsigned>(trial + 2);
        std::mt19937_64 random(seed);
        for (size_t row = 0; row < rows; ++row) {
            auto a = input.get() + n * row;
            std::iota(a, a + n, Key{0});
            std::shuffle(a, a + n, random);
        }
        for (int stage = 0; stage < count; ++stage) {
            // Balance both rotation and direction over all timed repetitions.
            const int slot = trial < 0 ? (stage + trial + count) % count :
                ((trial / count) % 2 == 0 ? (stage + trial) % count :
                 (trial - stage + count) % count);
            const int s = hierarchical ? selected[slot] : slot;
            std::memcpy(data.get(), input.get(), sizeof(Key) * n * rows);
            const auto start = Clock::now();
            for (size_t row = 0; row < rows; ++row) {
                Key* a = data.get() + n * row;
                layer(a, order[s].data(), k);
                if (tail) insert_tail(a, n);
            }
            const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(Clock::now() - start).count();
            if (tail) {
                for (size_t i = 0; i < n * rows; ++i) {
                    if (data[i] != i % n) throw std::runtime_error("wrong final rank");
                }
            } else {
                for (size_t i = 0; i < k; ++i) {
                    const auto j = k + order[s][i];
                    if (data[i] != std::min(input[i], input[j]) ||
                        data[j] != std::max(input[i], input[j]))
                        throw std::runtime_error("wrong gate output");
                }
            }
            if (trial >= 0) {
                std::cout << (tail ? "complete" : "layer") << ',' << n << ',' << k << ','
                    << rows << ',' << trial << ',' << stage << ',' << seed << ','
                    << names[s] << ',' << ns << '\n';
            }
        }
    }
}

int main(int argc, char** argv) {
    DWORD_PTR allowed = 0, system = 0;
    if (!GetProcessAffinityMask(GetCurrentProcess(), &allowed, &system) || !allowed)
        throw std::runtime_error("could not get processor affinity");
    const DWORD_PTR first = allowed & (~allowed + 1);
    if (!SetThreadAffinityMask(GetCurrentThread(), first))
        throw std::runtime_error("could not pin benchmark thread");
    std::cerr << "MSVC " << _MSC_FULL_VER << ", logical processor " << GetCurrentProcessorNumber()
              << ", pinned mask " << first << ", key bytes " << sizeof(Key) << '\n';
    std::cout << "scope,n,k,rows,trial,ordering_position,seed,layout,elapsed_ns\n";
    if (argc == 2 && std::string(argv[1]) == "--hierarchy") {
        run(8388608, 1, false, true);
        return 0;
    }
    for (size_t n : {65536u, 1048576u, 8388608u}) run(n, 1, false);
    run(256, 128, true);
    run(1024, 32, true);
}
