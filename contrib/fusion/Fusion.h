/**
 * @file Fusion.h
 * @author Seb Madgwick
 * @brief Main header file for the Fusion library.  This is the only file that
 * needs to be included when using the library.
 */

#ifndef FUSION_H
#define FUSION_H

//------------------------------------------------------------------------------
// Includes

#ifdef ARDUINO
#include <Arduino.h>
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

#include "FusionAhrs.h"
#include "FusionAxes.h"
#include "FusionCalibration.h"
#include "FusionCompass.h"
#include "FusionConvention.h"
#include "FusionMath.h"
#include "FusionOffset.h"

#ifdef __cplusplus
}
#endif

#endif
//------------------------------------------------------------------------------
// End of file
