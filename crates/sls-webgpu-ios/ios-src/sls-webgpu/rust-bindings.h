#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum slsAppError
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  slsAppError_CouldNotCreate,
};
#ifndef __cplusplus
typedef uint8_t slsAppError;
#endif // __cplusplus

typedef struct slsApp slsApp;

typedef enum slsAppResult_Tag {
  slsAppResult_Ok,
  slsAppResult_Err,
} slsAppResult_Tag;

typedef struct slsAppResult {
  slsAppResult_Tag tag;
  union {
    struct {
      struct slsApp *ok;
    };
    struct {
      slsAppError err;
    };
  };
} slsAppResult;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct slsAppResult sls_app_make(void);

void sls_app_release(struct slsApp *app);

int32_t sls_app_num(const struct slsApp *app);

int32_t get_cpu_count(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
