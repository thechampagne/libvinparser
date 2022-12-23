# libvinparser

[![](https://img.shields.io/github/v/tag/thechampagne/libvinparser?label=version)](https://github.com/thechampagne/libvinparser/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libvinparser)](https://github.com/thechampagne/libvinparser/blob/main/LICENSE)

Vehicle Identification Number (VIN) parser and validator.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libvinparser.git
```
#### 2. Navigate to the root
```
cd libvinparser
```
#### 3. Build the project
```
cargo build
```

### API

```c
typedef struct {
  char expected;
  char received;
} vinparser_checksum_error_info_t;

typedef struct {
  char* vin;
  char* country;
  char* manufacturer;
  char* region;
  int valid_checksum;
  vinparser_checksum_error_info_t checksum_error_info;
} vinparser_vin_t;

typedef enum {
    VINPARSER_ERROR_INCORRECTLENGTH,
    VINPARSER_ERROR_INVALIDCHARACTERS,
    VINPARSER_ERROR_CHECKSUMERROR,
} vinparser_error_t;

int vinparser_check_validity(const char* vin, vinparser_error_t* error_code);

int vinparser_get_info(const char* vin, vinparser_vin_t* vinparser_vin, vinparser_error_t* error_code);

int vinparser_verify_checksum(const char* vin, vinparser_error_t* error_code);

void vinparser_vin_clean(vinparser_vin_t* vinparser_vin);
```

### References
 - [vinparser](https://github.com/maybe-hello-world/vin_parser)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libvinparser/blob/main/LICENSE).
