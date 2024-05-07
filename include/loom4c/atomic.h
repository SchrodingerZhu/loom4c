#ifndef LOOM4C_ATOMIC_H
#define LOOM4C_ATOMIC_H
#include "ffi.h"

namespace loom {
template <class T> class Atomic {
  atm_t val_;

  constexpr static loom_size_type TYPE =
      static_cast<loom_size_type>(sizeof(T) * 8);

public:
  explicit Atomic(T val) : val_(loom_atomic_init(&val, TYPE)) {}
  T load(enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_load(val_, &res, TYPE, order);
    return res;
  }
  void store(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    loom_atomic_store(val_, &val, TYPE, order);
  }
  T fetch_add(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_add(val_, &val, &res, TYPE, order);
    return res;
  }
  T fetch_sub(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_sub(val_, &val, &res, TYPE, order);
    return res;
  }
  T fetch_and(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_and(val_, &val, &res, TYPE, order);
    return res;
  }
  T fetch_or(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_or(val_, &val, &res, TYPE, order);
    return res;
  }
  T fetch_xor(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_xor(val_, &val, &res, TYPE, order);
    return res;
  }
  T fetch_nand(T val,
               enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_nand(val_, &val, &res, TYPE, order);
    return res;
  }
  T fetch_max(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_max(val_, &val, &res, TYPE, order);
    return res;
  }
  T fetch_min(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    T res;
    loom_atomic_fetch_min(val_, &val, &res, TYPE, order);
    return res;
  }
  T swap(T val, enum loom_memory_order order = loom_memory_order::SeqCst) {
    loom_atomic_swap(val_, &val, TYPE, order);
    return val;
  }
  bool
  compare_exchange(T &expected, T desired,
                   enum loom_memory_order order = loom_memory_order::SeqCst) {
    return loom_atomic_compare_and_exchange(val_, &expected, &desired, TYPE,
                                            order, order);
  }
  bool compare_exchange(T &expected, T desired, enum loom_memory_order success,
                        enum loom_memory_order failure) {
    return loom_atomic_compare_and_exchange(val_, &expected, &desired, TYPE,
                                            success, failure);
  }
  void wait(T val) { loom_atomic_wait(val_, &val, TYPE); }
  bool notify_one() { return loom_atomic_notify_one(val_, TYPE); }
  size_t notify_all() { return loom_atomic_notify_all(val_, TYPE); }
  ~Atomic() { loom_atomic_destroy(val_, TYPE); }
};
} // namespace loom
#endif // LOOM4C_ATOMIC_H