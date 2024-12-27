#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Represents an in-progress CRC-64 computation.
struct Digest;

/// Begin functionality for building a C-compatible library
///
/// Opaque type for C for use in FFI
struct DigestHandle {
  Digest *_0;
};

extern "C" {

DigestHandle *digest_new();

/// # Safety
///
/// Uses unsafe method calls
void digest_write(DigestHandle *handle, const char *data, uintptr_t len);

/// # Safety
///
/// Uses unsafe method calls
uint64_t digest_sum64(const DigestHandle *handle);

/// # Safety
///
/// Uses unsafe method calls
void digest_free(DigestHandle *handle);

}  // extern "C"
