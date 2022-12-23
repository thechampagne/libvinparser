/*
 * Copyright (c) 2022 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#ifndef __VINPARSER_H__
#define __VINPARSER_H__

#ifdef __cplusplus
extern "C" {
#endif

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

extern int vinparser_check_validity(const char* vin, vinparser_error_t* error_code);

extern int vinparser_get_info(const char* vin, vinparser_vin_t* vinparser_vin, vinparser_error_t* error_code);

extern int vinparser_verify_checksum(const char* vin, vinparser_error_t* error_code);

extern void vinparser_vin_clean(vinparser_vin_t* vinparser_vin);

#ifdef __cplusplus
}
#endif

#endif // __VINPARSER_H__
