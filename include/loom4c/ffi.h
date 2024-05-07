#ifndef LOOM4C_FFI_H

#include <stdbool.h>

#ifdef __cplusplus
namespace loom {
extern "C" {
#endif

#define LOOM_ATOMIC_SIZE 8
#define LOOM_ATOMIC_ALIGN 8

typedef struct thread *thd_t;

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

void loom_atomic_load(void *__restrict, void *__restrict, enum loom_size_type,
                      enum loom_memory_order);

void loom_atomic_store(void *__restrict, void *__restrict, enum loom_size_type,
                       enum loom_memory_order);
void loom_atomic_init(void *__restrict, void *__restrict, enum loom_size_type);
void loom_atomic_destroy(void *__restrict, enum loom_size_type);
void loom_atomic_fetch_add(void *__restrict, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_sub(void *__restrict, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_and(void *__restrict, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_or(void *__restrict, void *__restrict, void *__restrict,
                          enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_xor(void *__restrict, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_nand(void *__restrict, void *__restrict,
                            void *__restrict, enum loom_size_type,
                            enum loom_memory_order);
void loom_atomic_fetch_max(void *__restrict, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_atomic_fetch_min(void *__restrict, void *__restrict, void *__restrict,
                           enum loom_size_type, enum loom_memory_order);
void loom_yield_now(void);

void loom_fence(enum loom_memory_order);

void loom_atomic_swap(void *__restrict, void *__restrict, enum loom_size_type,
                      enum loom_memory_order);
bool loom_atomic_compare_and_exchange(void *__restrict, void *__restrict,
                                      void *__restrict, enum loom_size_type,
                                      enum loom_memory_order,
                                      enum loom_memory_order);
bool loom_wait(void *__restrict, unsigned int);
unsigned int loom_wake(void *__restrict, unsigned int);

#ifdef __cplusplus
}
}
#endif

#define LOOM4C_FFI_H
#endif // LOOM4C_FFI_H