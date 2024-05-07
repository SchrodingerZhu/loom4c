#ifndef LOOM4C_FFI_H
#define LOOM4C_FFI_H

#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
namespace loom {
extern "C" {
#endif

typedef struct __loom_thread_s *thd_t;

void loom_start(void (*)());
thd_t loom_create_thread(void *(*)(void *), void *);
void *loom_join_thread(thd_t);

enum loom_size_type {
  AtomicU8 = 8,
  AtomicU16 = 16,
  AtomicU32 = 32,
  AtomicU64 = 64,
};

enum loom_memory_order {
  Relaxed = 0,
  Acquire = 1,
  Release = 2,
  AcqRel = 3,
  SeqCst = 4,
};

typedef struct __loom_atomic_handle_s *atm_t;

void loom_atomic_load(atm_t, void *__restrict, enum loom_size_type,
                      enum loom_memory_order);

void loom_atomic_store(atm_t, void *__restrict, enum loom_size_type,
                       enum loom_memory_order);
atm_t loom_atomic_init(void *__restrict, enum loom_size_type);
void loom_atomic_destroy(atm_t, enum loom_size_type);
void loom_atomic_fetch_add(atm_t, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_sub(atm_t, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_and(atm_t, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_or(atm_t, void *__restrict, void *__restrict,
                          enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_xor(atm_t, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_nand(atm_t, void *__restrict, void *__restrict,
                            enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_max(atm_t, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_min(atm_t, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_yield_now(void);

void loom_fence(enum loom_memory_order);

void loom_atomic_swap(atm_t, void *__restrict, enum loom_size_type,
                      enum loom_memory_order);
bool loom_atomic_compare_and_exchange(atm_t, void *__restrict, void *__restrict,
                                      enum loom_size_type,
                                      enum loom_memory_order,
                                      enum loom_memory_order);
void loom_atomic_wait(atm_t, void *__restrict, enum loom_size_type);
bool loom_atomic_notify_one(atm_t, enum loom_size_type);
size_t loom_atomic_notify_all(atm_t, enum loom_size_type);
void loom_spin_loop_hint(void);

#ifdef __cplusplus
}
}
#endif

#endif // LOOM4C_FFI_H