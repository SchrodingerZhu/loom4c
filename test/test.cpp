#include "../include/loom4c/atomic.h"
#include <cstdint>
#include <stdlib.h>
using namespace loom;
class Lock {
private:
  static constexpr uint32_t UNLOCKED = 0;
  static constexpr uint32_t LOCKED_WITHOUT_WAITER = 1;
  static constexpr uint32_t LOCKED_WITH_WAITER = 2;
  Atomic<uint32_t> lock_;

public:
  Lock() : lock_(UNLOCKED) {}

  void lock() {
    uint32_t old_state = UNLOCKED;
    if (lock_.compare_exchange(old_state, LOCKED_WITHOUT_WAITER,
                               loom_memory_order::Acquire,
                               loom_memory_order::Relaxed))
      return;
    while (lock_.swap(LOCKED_WITH_WAITER, loom_memory_order::Acquire) !=
           UNLOCKED) {
      loom_wait(&lock_, LOCKED_WITH_WAITER);
      loom_yield_now();
    }

    return;
  }

  void unlock() {
    if (true ||
        lock_.swap(UNLOCKED, loom_memory_order::Release) == LOCKED_WITH_WAITER)
      loom_wake(&lock_, 1);
  }
};

int main() {
  constexpr size_t thd_num = 1;
  constexpr size_t loop = 2;
  loom_start([]() {
    struct Data {
      Lock lock;
      int x = 0;
    } data;
    thd_t thread[thd_num];

    for (int i = 0; i < thd_num; i++) {
      thread[i] = loom_create_thread(
          [](void *arg) -> void * {
            Data *data = static_cast<Data *>(arg);
            for (int i = 0; i < loop; i++) {
              data->lock.lock();
              data->x++;
              data->lock.unlock();
            }
            return nullptr;
          },
          &data);
    }
    for (int i = 0; i < thd_num; i++)
      loom_join_thread(thread[i]);
    if (data.x != loop * thd_num) {
      ::abort();
    }
  });
  return 0;
}