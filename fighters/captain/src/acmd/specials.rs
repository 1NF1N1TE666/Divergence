use super::*;
use config::Special;

const SPECIALN_2_RATES: [f32; 115] = [
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 1.0, 1.002, 1.01, 1.016, 1.025, 1.032, 1.039, 1.049,
    1.062, 1.072, 1.081, 1.093, 1.109, 1.123, 1.134, 1.143, 1.151, 1.158,
    1.164, 1.169, 1.174, 1.179, 1.184, 1.188, 1.191, 1.193, 1.196, 1.2,
    1.204, 1.207, 1.208, 1.209, 1.208, 1.204, 1.196, 1.172, 1.126, 1.07,
    1.032, 1.021, 1.018, 1.013, 1.005, 1.0, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 1.015,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
];

const SPECIALN_3_RATES: [f32; 115] = [
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.999, 1.005, 1.02, 1.048, 1.074, 1.095, 1.116, 1.148,
    1.184, 1.215, 1.242, 1.28, 1.326, 1.369, 1.403, 1.429, 1.453, 1.476,
    1.493, 1.507, 1.521, 1.537, 1.552, 1.564, 1.573, 1.58, 1.589, 1.601,
    1.612, 1.62, 1.624, 1.626, 1.624, 1.613, 1.587, 1.516, 1.377, 1.209,
    1.095, 1.063, 1.059, 1.04, 1.015, 1.001, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.991, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
];

const SPECIALAIRN_2_RATES: [f32; 124] = [
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.999, 1.006, 1.019, 1.044, 1.059, 1.064, 1.064, 1.064,
    1.064, 1.065, 1.066, 1.068, 1.07, 1.069, 1.067, 1.066, 1.07, 1.08,
    1.097, 1.119, 1.14, 1.157, 1.174, 1.192, 1.21, 1.225, 1.238, 1.248,
    1.255, 1.258, 1.256, 1.25, 1.242, 1.231, 1.214, 1.182, 1.131, 1.075,
    1.042, 1.04, 1.044, 1.028, 1.01, 1.001, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 1.008, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
];

const SPECIALAIRN_3_RATES: [f32; 124] = [
    0.993, 0.993, 0.993, 0.993, 0.993, 0.993, 0.993, 0.993, 0.993, 0.993,
    0.993, 0.993, 0.997, 1.016, 1.071, 1.133, 1.177, 1.191, 1.192, 1.193,
    1.193, 1.193, 1.198, 1.205, 1.21, 1.207, 1.201, 1.199, 1.21, 1.24,
    1.292, 1.357, 1.419, 1.471, 1.521, 1.577, 1.631, 1.676, 1.714, 1.746,
    1.767, 1.775, 1.768, 1.75, 1.726, 1.694, 1.643, 1.548, 1.393, 1.224,
    1.126, 1.118, 1.122, 1.083, 1.03, 1.003, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.978, 0.999, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.001,
];

const SPECIALNTURN_2_RATES: [f32; 109] = [
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.999, 1.007,
    1.03, 1.057, 1.08, 1.093, 1.106, 1.124, 1.148, 1.172, 1.191, 1.204,
    1.212, 1.216, 1.222, 1.23, 1.242, 1.257, 1.272, 1.287, 1.298, 1.303,
    1.302, 1.291, 1.259, 1.193, 1.11, 1.054, 1.039, 1.038, 1.025, 1.009,
    1.001, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.998, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

const SPECIALNTURN_3_RATES: [f32; 109] = [
    0.994, 0.994, 0.994, 0.994, 0.994, 0.994, 0.994, 0.994, 0.994, 0.994,
    0.994, 0.994, 0.994, 0.994, 0.994, 0.994, 0.994, 0.994, 0.998, 1.022,
    1.088, 1.17, 1.239, 1.279, 1.317, 1.372, 1.444, 1.517, 1.574, 1.613,
    1.635, 1.649, 1.665, 1.69, 1.726, 1.77, 1.818, 1.862, 1.895, 1.91,
    1.906, 1.875, 1.779, 1.578, 1.329, 1.161, 1.117, 1.11, 1.074, 1.027,
    1.003, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

const SPECIALAIRNTURN_2_RATES: [f32; 119] = [
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998,
    0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.999, 1.006,
    1.031, 1.054, 1.082, 1.104, 1.122, 1.136, 1.147, 1.157, 1.168, 1.18,
    1.194, 1.209, 1.226, 1.245, 1.263, 1.279, 1.29, 1.297, 1.299, 1.297,
    1.291, 1.278, 1.244, 1.179, 1.102, 1.055, 1.049, 1.053, 1.034, 1.013,
    1.001, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 1.004, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

const SPECIALAIRNTURN_3_RATES: [f32; 119] = [
    0.995, 0.995, 0.995, 0.995, 0.995, 0.995, 0.995, 0.995, 0.995, 0.995,
    0.995, 0.995, 0.995, 0.995, 0.995, 0.995, 0.995, 0.995, 0.998, 1.019,
    1.073, 1.161, 1.245, 1.312, 1.365, 1.407, 1.441, 1.471, 1.504, 1.541,
    1.583, 1.627, 1.677, 1.734, 1.791, 1.839, 1.872, 1.892, 1.899, 1.892,
    1.874, 1.834, 1.733, 1.536, 1.305, 1.165, 1.147, 1.153, 1.103, 1.038,
    1.004, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997,
    0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997,
    0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997,
    0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997,
    0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997,
    0.997, 0.997, 0.997, 1.011, 0.999, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.001,
];

const SPECIALSSTART_2_RATES: [f32; 91] = [
    1.0, 0.934, 0.88, 0.806, 0.71, 0.63, 0.605, 0.657, 0.778, 0.938,
    1.022, 1.04, 1.01, 1.0, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999,
    0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 1.003, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0,
];

const SPECIALSSTART_3_RATES: [f32; 91] = [
    0.637, 0.637, 0.637, 0.637, 0.703, 1.163, 2.586, 5.334, 5.713, 3.953,
    1.334, 0.666, 0.698, 0.588, 0.451, 0.366, 0.341, 0.352, 0.377, 0.398,
    0.405, 0.405, 0.418, 0.456, 0.528, 0.628, 0.741, 0.848, 0.938, 0.985,
    1.0, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002,
    1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002,
    1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002,
    1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002,
    1.002, 1.002, 1.002, 0.993, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0,
];

const SPECIALSEND_2_RATES: [f32; 47] = [
    1.0, 0.837, 0.768, 0.768, 0.794, 0.833, 0.891, 0.932, 0.977, 1.029,
    1.066, 1.081, 1.084, 1.085, 1.087, 1.089, 1.092, 1.095, 1.099, 1.104,
    1.11, 1.118, 1.128, 1.141, 1.155, 1.162, 1.145, 1.107, 1.068, 1.046,
    1.038, 1.035, 1.034, 1.03, 1.011, 0.961, 0.883, 0.82, 0.794, 0.794,
    0.799, 0.798, 0.79, 0.768, 0.716, 0.616, 0.472,
];

const SPECIALAIRSSTART_3_RATES: [f32; 79] = [
    0.837, 0.837, 0.837, 0.837, 0.837, 0.874, 1.116, 1.825, 3.053, 3.81,
    3.999, 3.877, 3.514, 2.747, 1.632, 1.04, 0.84, 0.78, 0.708, 0.639,
    0.537, 0.469, 0.434, 0.417, 0.409, 0.403, 0.398, 0.394, 0.394, 0.4,
    0.416, 0.448, 0.512, 0.618, 0.748, 0.864, 0.948, 0.988, 1.0, 1.002,
    1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002,
    1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002,
    1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002,
    1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 1.002, 0.986,
];

const SPECIALAIRSEND_2_RATES: [f32; 64] = [
    1.001, 1.001, 1.001, 1.0, 0.997, 0.985, 1.015, 1.088, 1.154, 1.209,
    1.249, 1.275, 1.288, 1.294, 1.297, 1.297, 1.297, 1.295, 1.291, 1.285,
    1.278, 1.271, 1.262, 1.254, 1.244, 1.226, 1.189, 1.139, 1.094, 1.069,
    1.057, 1.05, 1.046, 1.054, 1.082, 1.124, 1.163, 1.193, 1.219, 1.246,
    1.277, 1.306, 1.321, 1.298, 1.219, 1.107, 1.027, 0.997, 0.992, 0.992,
    0.992, 0.992, 0.992, 0.992, 0.992, 0.992, 0.992, 0.992, 0.992, 0.992,
    0.992, 0.992, 0.992, 0.989,
];

const SPECIALHI_3_RATES: [f32; 66] = [
    1.305, 1.895, 2.458, 2.886, 3.114, 3.197, 3.206, 3.156, 3.008, 2.717,
    2.3, 1.845, 1.415, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

const SPECIALLW_2_RATES: [f32; 38] = [
    1.131, 1.248, 1.325, 1.348, 1.32, 1.263, 1.199, 1.144, 1.104, 1.078,
    1.063, 1.057, 1.062, 1.078, 1.08, 1.056, 1.03, 1.019, 1.021, 1.025,
    1.028, 1.03, 1.031, 1.031, 1.029, 1.027, 1.025, 1.024, 1.022, 1.021,
    1.021, 1.02, 1.019, 1.021, 3.4, 4.4, 3.2, 1.98,
];

const SPECIALLW_3_RATES: [f32; 38] = [
    1.371, 1.717, 1.946, 2.012, 1.931, 1.759, 1.569, 1.412, 1.296, 1.221,
    1.181, 1.164, 1.174, 1.182, 1.065, 0.75, 0.506, 0.377, 0.367, 0.452,
    0.487, 0.478, 0.467, 0.472, 0.479, 0.465, 0.428, 0.414, 0.425, 0.445,
    0.462, 0.476, 0.492, 0.508, 0.521, 0.529, 0.7, 0.98,
];

const SPECIALAIRLW_3_RATES: [f32; 29] = [
    1.175, 1.415, 1.665, 1.842, 1.893, 1.869, 1.837, 1.82, 1.82, 1.823,
    1.785, 1.656, 1.463, 1.294, 1.165, 0.978, 0.698, 0.517, 0.437, 0.415,
    0.433, 0.485, 0.469, 0.44, 0.475, 0.554, 0.577, 0.512, 0.468,
];

const SPECIALLWEND_3_RATES: [f32; 47] = [
    1.25, 1.238, 1.25, 1.272, 1.285, 1.319, 1.432, 1.65, 1.938, 2.221,
    2.432, 2.555, 2.626, 2.682, 2.733, 2.782, 2.826, 2.846, 2.813, 2.687,
    2.409, 1.954, 1.428, 1.113, 1.003, 0.984, 0.982, 0.981, 0.981, 0.983,
    0.995, 1.033, 1.045, 1.045, 1.042, 1.039, 1.04, 1.046, 1.053, 1.055,
    1.053, 1.048, 1.045, 1.046, 1.053, 1.068, 1.089,
];

const SPECIALAIRLW_2_RATES: [f32; 29] = [
    0.964, 0.915, 0.863, 0.827, 0.817, 0.821, 0.828, 0.832, 0.832, 0.831,
    0.839, 0.865, 0.905, 0.938, 0.955, 0.968, 0.986, 0.999, 1.005, 1.006,
    1.004, 1.001, 1.0, 1.0, 1.0, 1.0, 0.999, 1.0, 1.0,
];

const SPECIALAIRLWEND_2_RATES: [f32; 45] = [
    1.0, 0.815, 0.765, 0.84, 0.895, 0.913, 0.903, 0.868, 0.807, 0.736,
    0.686, 0.665, 0.659, 0.658, 0.657, 0.657, 0.658, 0.66, 0.662, 0.666,
    0.669, 0.671, 0.672, 0.673, 0.672, 0.671, 0.67, 0.67, 0.683, 0.724,
    0.787, 0.846, 0.88, 0.893, 0.896, 0.9, 0.911, 0.93, 0.951, 0.967,
    0.976, 0.979, 0.977, 0.969, 0.953,
];


const DASH_SPEEDS: [f32; 37] = [
    4.765, 4.607, 4.449, 4.185, 3.922, 3.658, 3.394, 3.164, 2.966, 2.768,
    2.57, 2.359, 2.133, 1.907, 1.681, 1.516, 1.351, 1.186, 1.063, 0.939,
    0.816, 0.692, 0.631, 0.57, 0.51, 0.449, 0.388, 0.327, 0.28, 0.247,
    0.214, 0.181, 0.148, 0.115, 0.082, 0.049, 0.016,
];

const GROUND_HOP_SPEEDS: [f32; 8] = [
    0.0, -3.772, -4.321, -4.134, -2.618, -0.121, 0.0, 0.0,
];

const AIR_HOP_SPEEDS: [f32; 10] = [
    -1.02, -1.638, -1.583, -1.525, -1.346, -0.978, -0.369, 0.0, 0.0, 0.0,
];

unsafe fn dash_start(agent: &mut L2CAgentBase) {
    let ground = agent.is_situation(*SITUATION_KIND_GROUND);
    let reset = if ground {*ENERGY_STOP_RESET_TYPE_GROUND} else {*ENERGY_STOP_RESET_TYPE_AIR};
    sv_kinetic_energy!(reset_energy, agent, FIGHTER_KINETIC_ENERGY_ID_STOP, reset, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
}

unsafe fn dash_end(agent: &mut L2CAgentBase) {
    sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
}

unsafe fn dash_curve(agent: &mut L2CAgentBase, rates: &[f32], speeds: &[f32], start: f32, from: f32, until: f32) {
    let mut f = from;
    while f < until {
        frame(agent.lua_state_agent, f);
        FT_MOTION_RATE(agent, rates[f as usize]);
        if is_excute(agent) {
            let speed = speeds[(f - start) as usize] * PostureModule::lr(agent.module_accessor);
            sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_STOP, speed, 0.0);
        }
        f += 1.0;
    }
    agent.boma().rate_curve(agent, rates, until);
}

unsafe extern "C" fn game_specialn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 53.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        dash_start(agent);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 20.0, 361, 59, 0, 104, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 20.0, 361, 59, 0, 94, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 20.0, 361, 59, 0, 84, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALN_2_RATES, &DASH_SPEEDS, 53.0, 53.0, 58.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 16.0, 361, 59, 0, 90, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 16.0, 361, 59, 0, 80, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 16.0, 361, 59, 0, 70, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALN_2_RATES, &DASH_SPEEDS, 53.0, 58.0, 62.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 13.0, 361, 59, 0, 78, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 13.0, 361, 59, 0, 68, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 361, 59, 0, 58, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALN_2_RATES, &DASH_SPEEDS, 53.0, 62.0, 66.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 11.0, 361, 59, 0, 71, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 11.0, 361, 59, 0, 61, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 11.0, 361, 59, 0, 51, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALN_2_RATES, &DASH_SPEEDS, 53.0, 66.0, 72.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    dash_curve(agent, &SPECIALN_2_RATES, &DASH_SPEEDS, 53.0, 72.0, 90.0);
    if is_excute(agent) {
        dash_end(agent);
    }
    agent.boma().rate_curve(agent, &SPECIALN_2_RATES, 115.0);
}

unsafe extern "C" fn game_specialn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 53.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 82, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 22.0, 361, 59, 0, 82, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 82, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 18.0, 361, 100, 0, 0, 15.0, 5.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 58.0);
    if is_excute(agent) {
        KineticModule::clear_speed_all(boma);
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 9.0, 361, 100, 0, 0, 15.0, 3.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 63.0);
    if is_excute(agent) {
        ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 90, 100, 0, 30, 8.0, 0.0, 30.0, 10.0, Some(0.0), Some(45.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ADD_SPEED_NO_LIMIT(agent, 0.4, 0.0);
    }
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 66.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::clear_speed_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALN_3_RATES, 115.0);
}

unsafe extern "C" fn game_specialnturn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALNTURN_2_RATES, 21.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    agent.boma().rate_curve(agent, &SPECIALNTURN_2_RATES, 48.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        dash_start(agent);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 65, 0, 97, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 22.0, 361, 65, 0, 87, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 65, 0, 77, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALNTURN_2_RATES, &DASH_SPEEDS, 48.0, 48.0, 53.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 18.0, 361, 65, 0, 85, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 18.0, 361, 65, 0, 75, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 361, 65, 0, 65, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALNTURN_2_RATES, &DASH_SPEEDS, 48.0, 53.0, 57.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 15.0, 361, 65, 0, 76, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 15.0, 361, 65, 0, 66, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 15.0, 361, 65, 0, 56, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALNTURN_2_RATES, &DASH_SPEEDS, 48.0, 57.0, 61.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 13.0, 361, 65, 0, 69, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 13.0, 361, 65, 0, 59, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 361, 65, 0, 49, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    dash_curve(agent, &SPECIALNTURN_2_RATES, &DASH_SPEEDS, 48.0, 61.0, 67.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    dash_curve(agent, &SPECIALNTURN_2_RATES, &DASH_SPEEDS, 48.0, 67.0, 85.0);
    if is_excute(agent) {
        dash_end(agent);
    }
    agent.boma().rate_curve(agent, &SPECIALNTURN_2_RATES, 109.0);
}

unsafe extern "C" fn game_specialnturn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALNTURN_3_RATES, 21.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    agent.boma().rate_curve(agent, &SPECIALNTURN_3_RATES, 48.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 25.0, 361, 65, 0, 76, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 25.0, 361, 65, 0, 76, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 25.0, 361, 65, 0, 76, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 18.0, 361, 100, 0, 0, 15.0, 5.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALNTURN_3_RATES, 54.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALNTURN_3_RATES, 59.0);
    if is_excute(agent) {
        ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 90, 100, 0, 30, 8.0, 0.0, 30.0, -8.0, Some(0.0), Some(45.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALNTURN_3_RATES, 61.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALNTURN_3_RATES, 109.0);
}

unsafe extern "C" fn game_specialairn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 51.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 53.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 17.0, 361, 59, 0, 72, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 17.0, 361, 59, 0, 72, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 17.0, 361, 59, 0, 72, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 58.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 15.0, 361, 59, 0, 93, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 15.0, 361, 59, 0, 83, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 15.0, 361, 59, 0, 73, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 62.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 13.0, 361, 59, 0, 85, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 13.0, 361, 59, 0, 75, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 361, 59, 0, 65, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 66.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 11.0, 361, 59, 0, 76, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 11.0, 361, 59, 0, 66, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 11.0, 361, 59, 0, 56, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 72.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 77.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 2, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_2_RATES, 124.0);
}

unsafe extern "C" fn game_specialairn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 51.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 53.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 19.0, 361, 59, 0, 80, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 19.0, 361, 59, 0, 80, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 19.0, 361, 59, 0, 80, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 18.0, 361, 100, 0, 0, 15.0, 5.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 59.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 62.0);
    if is_excute(agent) {
        ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 90, 100, 0, 30, 8.0, 0.0, 30.0, 8.0, Some(0.0), Some(45.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 64.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 71.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 2, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRN_3_RATES, 124.0);
}

unsafe extern "C" fn game_specialairnturn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 21.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 48.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 20.0, 361, 59, 0, 74, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 20.0, 361, 59, 0, 74, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 20.0, 361, 59, 0, 74, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 53.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 18.0, 361, 59, 0, 97, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 18.0, 361, 59, 0, 87, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 361, 59, 0, 77, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 57.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 16.0, 361, 59, 0, 90, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 16.0, 361, 59, 0, 80, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 16.0, 361, 59, 0, 70, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 61.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 14.0, 361, 59, 0, 82, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 14.0, 361, 59, 0, 72, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 14.0, 361, 59, 0, 62, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 67.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 72.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 2, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_2_RATES, 119.0);
}

unsafe extern "C" fn game_specialairnturn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 21.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(boma, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 48.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 82, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 22.0, 361, 59, 0, 82, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 82, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 18.0, 361, 100, 0, 0, 15.0, 5.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 54.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 57.0);
    if is_excute(agent) {
        ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 90, 100, 0, 30, 8.0, 0.0, 30.0, -8.0, Some(0.0), Some(45.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 59.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 66.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 2, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRNTURN_3_RATES, 119.0);
}

unsafe extern "C" fn effect_specialn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.11947, -0.78984, -0.5426, 1, true);
    }
    frame(lua_state, 33.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fp_hold"), false, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
}

unsafe extern "C" fn effect_specialn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.11947, -0.78984, -0.5426, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.65);
    }
    frame(lua_state, 33.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fp_hold"), false, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialnturn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.11947, -0.78984, -0.5426, 1, true);
    }
    frame(lua_state, 33.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fp_hold"), false, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
}

unsafe extern "C" fn effect_specialnturn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.11947, -0.78984, -0.5426, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.65);
    }
    frame(lua_state, 33.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fp_hold"), false, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_captain_001"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_n04"));
    }
    wait(lua_state, 33.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_special_n04"));
        PLAY_SE(agent, Hash40::new("vc_captain_002"));
        PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
    }
}

unsafe extern "C" fn sound_specialn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_captain_001"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_n04"));
    }
    wait(lua_state, 33.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_special_n04"));
        PLAY_SE(agent, Hash40::new("vc_captain_002"));
        PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
    }
}

unsafe extern "C" fn sound_specialnturn_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
        PLAY_SE(agent, Hash40::new("se_common_swing_08"));
    }
    wait(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_special_n04"));
        PLAY_SE(agent, Hash40::new("vc_captain_002"));
        PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
    }
}

unsafe extern "C" fn sound_specialnturn_mighty(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
        PLAY_SE(agent, Hash40::new("se_common_swing_08"));
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_special_n04"));
        PLAY_SE(agent, Hash40::new("vc_captain_002"));
        PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
    }
}

unsafe extern "C" fn game_specialsstart_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALSSTART_2_RATES, 9.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    agent.boma().rate_curve(agent, &SPECIALSSTART_2_RATES, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 9.0, 4.4, Some(0.0), Some(5.0), Some(4.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 12.0, 4.4, Some(0.0), Some(5.0), Some(4.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    agent.boma().rate_curve(agent, &SPECIALSSTART_2_RATES, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        JostleModule::set_status(boma, true);
    }
    agent.boma().rate_curve(agent, &SPECIALSSTART_2_RATES, 91.0);
}

unsafe extern "C" fn game_specialsstart_windup(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALSSTART_3_RATES, 2.0);
    if is_excute(agent) {
        dash_start(agent);
    }
    dash_curve(agent, &SPECIALSSTART_3_RATES, &GROUND_HOP_SPEEDS, 2.0, 2.0, 10.0);
    if is_excute(agent) {
        dash_end(agent);
        JostleModule::set_status(boma, false);
    }
    agent.boma().rate_curve(agent, &SPECIALSSTART_3_RATES, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 9.0, 8.8, Some(0.0), Some(5.0), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 12.0, 8.8, Some(0.0), Some(5.0), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    agent.boma().rate_curve(agent, &SPECIALSSTART_3_RATES, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        JostleModule::set_status(boma, true);
    }
    agent.boma().rate_curve(agent, &SPECIALSSTART_3_RATES, 91.0);
}

unsafe extern "C" fn game_specialairsstart_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BACK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.1, 0.0, 12.0, 11.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_GRAVITY_ONOFF);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairsstart_windup(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSSTART_3_RATES, 2.0);
    if is_excute(agent) {
        dash_start(agent);
    }
    dash_curve(agent, &SPECIALAIRSSTART_3_RATES, &AIR_HOP_SPEEDS, 2.0, 2.0, 12.0);
    if is_excute(agent) {
        dash_end(agent);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSSTART_3_RATES, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BACK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.1, 0.0, 12.0, 11.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSSTART_3_RATES, 25.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSSTART_3_RATES, 31.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_GRAVITY_ONOFF);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSSTART_3_RATES, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSSTART_3_RATES, 79.0);
}

unsafe extern "C" fn game_specialsend_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALSEND_2_RATES, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    agent.boma().rate_curve(agent, &SPECIALSEND_2_RATES, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handl"), 13.3, 85, 30, 0, 82, 6.0, 0.0, -2.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.3, 85, 30, 0, 82, 5.5, 0.0, 9.0, 5.5, Some(0.0), Some(9.0), Some(12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
    }
    agent.boma().rate_curve(agent, &SPECIALSEND_2_RATES, 7.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 13.3, 85, 30, 0, 82, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
    }
    agent.boma().rate_curve(agent, &SPECIALSEND_2_RATES, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    agent.boma().rate_curve(agent, &SPECIALSEND_2_RATES, 47.0);
}

unsafe extern "C" fn game_specialairsend_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSEND_2_RATES, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 15.0, 270, 70, 0, 80, 8.0, 8.0, 0.0, 0.4, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 15.0, 60, 80, 0, 80, 8.0, 8.0, 0.0, 0.4, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 15.0, 60, 80, 0, 80, 2.5, -2.0, 0.0, 0.0, Some(-5.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSEND_2_RATES, 6.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f{x: -0.5, y: 2.5, z: 0.0});
        ATTACK(agent, 2, 0, Hash40::new("top"), 15.0, 60, 80, 0, 80, 4.0, 0.0, 8.0, 5.5, Some(0.0), Some(1.0), Some(3.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSEND_2_RATES, 7.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSEND_2_RATES, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRSEND_2_RATES, 64.0);
}

unsafe extern "C" fn game_specialairsend_windup(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 10.0, 270, 70, 0, 60, 8.0, 8.0, 0.0, 0.4, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 10.0, 60, 80, 0, 60, 8.0, 8.0, 0.0, 0.4, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 10.0, 60, 80, 0, 60, 2.5, -2.0, 0.0, 0.0, Some(-5.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f{x: -0.5, y: 2.5, z: 0.0});
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 60, 80, 0, 60, 4.0, 0.0, 8.0, 5.5, Some(0.0), Some(1.0), Some(3.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn effect_specialsstart_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.45, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialsstart_windup(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.45);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 2, 0, 0, 0, 180, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairsstart_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.45, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairsstart_windup(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..6 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -5, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.4);
        }
        wait(lua_state, 4.0);
    }
}

unsafe extern "C" fn effect_specialsend_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fn"), Hash40::new("top"), 1, -4, 0, 0, 0, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairsend_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fn_air"), Hash40::new("top"), 0, 4, 3.5, 0, 20, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

unsafe extern "C" fn sound_specialsstart_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s02"));
    }
    wait(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s03"));
    }
    wait(lua_state, 36.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_step_right_m"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_step_left_m"));
    }
}

unsafe extern "C" fn sound_specialsstart_windup(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s02"));
    }
    wait(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s03"));
    }
    wait(lua_state, 36.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_step_right_m"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_step_left_m"));
    }
}

unsafe extern "C" fn sound_specialairsstart_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s02"));
    }
}

unsafe extern "C" fn sound_specialairsstart_windup(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s02"));
    }
}

unsafe extern "C" fn sound_specialsend_heavy(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_swing_l"));
    }
}

unsafe extern "C" fn game_specialhi_strike(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(agent, 0, 0, Hash40::new("arml"), 8.0, 75, 100, 0, 40, 3.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}

unsafe extern "C" fn game_specialhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        CATCH(agent, 0, Hash40::new("top"), 6.111, 0.0, 13.0, 7.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 6.5, 0.0, 8.8, 13.7, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_G);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 15.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 6.111, 0.0, 15.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 6.111, 0.0, 11.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *COLLISION_SITUATION_MASK_GA);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 31.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 36.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 50.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
    agent.boma().rate_curve(agent, &SPECIALHI_3_RATES, 66.0);
}

unsafe extern "C" fn effect_specialhi_strike(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_counter_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_COLOR(agent, 0.7, 0.75, 1.6);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 5, 4, 90, 0, 0, 0.7, true);
            LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 3.0);
    }
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_speedline"), false, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_counter_flash"), false, true);
    }
}

unsafe extern "C" fn effect_specialhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("captain_fd_hold"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 0.56, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fd_hold"), false, true);
    }
}

unsafe extern "C" fn effect_specialairhi_strike(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_counter_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_COLOR(agent, 0.7, 0.75, 1.6);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 5, 3, 90, 0, 0, 0.7, true);
            LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 3.0);
    }
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_speedline"), false, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_counter_flash"), false, true);
    }
}

unsafe extern "C" fn effect_specialairhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fd_hold"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fd_hold"), false, true);
    }
}

unsafe extern "C" fn sound_specialhi_strike(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_h02"));
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_captain_special_h01"));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_jump01"));
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_06"));
    }
}

unsafe extern "C" fn sound_specialhi_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_h02"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_captain_special_h01"));
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_jump01"));
    }
    wait(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
    wait(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_06"));
    }
}

unsafe extern "C" fn game_specialhicatch_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 10.0, 361, 100, 0, 50, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhithrow_explosive(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 16.25, 50, 60, 0, 125, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        CHECK_FINISH_CAMERA(agent, 2, 1);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let target_object = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_hit_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_hit_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target_object, target_hit_group, target_hit_no);
        ADD_SPEED_NO_LIMIT(agent, 0.0, 0.5);
    }
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
    frame(lua_state, 45.0);
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 59.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn game_speciallw_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 15.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 0.0, 366, 100, 50, 0, 5.0, 10.0, -2.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("legr"), 0.0, 366, 100, 40, 0, 5.0, 10.0, -2.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 1, Hash40::new("legr"), 1.154, 366, 100, 60, 0, 3.8, 10.0, -2.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 19.0);
    if is_excute(agent) {
        ATK_POWER(agent, 2, 1.0);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.077, 52, 110, 0, 60, 10.0, 0.0, 5.0, 10.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_2_RATES, 38.0);
}

unsafe extern "C" fn game_speciallw_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 15.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 13.846, 52, 88, 0, 40, 3.8, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 52, 60, 0, 40, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 6.0, 46, 65, 0, 40, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    agent.boma().rate_curve(agent, &SPECIALLW_3_RATES, 38.0);
}

unsafe extern "C" fn game_specialairlw_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 2.0, 366, 100, 40, 0, 5.76, 4.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 1.846, 366, 100, 40, 0, 5.76, 4.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 27.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 1.636);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 90, 95, 0, 100, 10.0, 0.0, 0.0, 5.0, Some(0.0), Some(5.0), Some(5.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_2_RATES, 29.0);
}

unsafe extern "C" fn game_specialairlw_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 14.0, 361, 57, 0, 80, 5.76, 4.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 11.077, 361, 46, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 8.182, 361, 46, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 29.0);
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLW_3_RATES, 29.0);
}

unsafe extern "C" fn game_specialairlwend_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRLWEND_2_RATES, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 1, 1, Hash40::new("top"), 5.0, 70, 35, 0, 80, 6.0, 0.0, 2.0, 10.0, Some(0.0), Some(2.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLWEND_2_RATES, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLWEND_2_RATES, 45.0);
}

unsafe extern "C" fn game_specialairlwend_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALAIRLWEND_2_RATES, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 60, 49, 0, 94, 7.0, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(-5.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLWEND_2_RATES, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    agent.boma().rate_curve(agent, &SPECIALAIRLWEND_2_RATES, 45.0);
}

unsafe extern "C" fn game_speciallwendair_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_FALL_ONOFF);
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_CLIFF_CHECK);
    }
}

unsafe extern "C" fn game_speciallwendair_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        KineticModule::clear_speed_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_FALL_ONOFF);
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_CLIFF_CHECK);
    }
}

unsafe extern "C" fn game_speciallwend_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    agent.boma().rate_curve(agent, &SPECIALLWEND_3_RATES, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_SP_BRAKE);
    }
    agent.boma().rate_curve(agent, &SPECIALLWEND_3_RATES, 31.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_SP_BRAKE);
    }
    agent.boma().rate_curve(agent, &SPECIALLWEND_3_RATES, 47.0);
}

unsafe extern "C" fn effect_speciallw_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fk_hold"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fk"), Hash40::new("top"), 0, 8, 17, -90, 0, 160, 0.8, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.35);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), -2, 8, 12, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 13.0);
    for _ in 0..4 {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, -4, 0, 0, 0, 1, 2, 0, 5, 0, 0, 0, false);
        }
        wait(lua_state, 6.0);
    }
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, true);
    }
}

unsafe extern "C" fn effect_speciallw_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 0.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("captain_fk"), Hash40::new("top"), 0, 8, 17, -90, 0, 160, 0.63, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.35);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, -4, 0, 0, 0, 1, 2, 0, 5, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, -4, 0, 0, 0, 1, 2, 0, 5, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, -4, 0, 0, 0, 1, 2, 0, 5, 0, 0, 0, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
}

unsafe extern "C" fn effect_specialairlw_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fk_hold"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fk_air"), Hash40::new("toel"), -1, -1, 0, 0, 0, 140, 0.9, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.35);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), -2, -5, 6, 0, 0, 0, 1.1, true);
    }
    frame(lua_state, 13.0);
    for _ in 0..4 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
        }
        wait(lua_state, 4.0);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, true);
    }
}

unsafe extern "C" fn effect_specialairlw_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 0.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("captain_fk_air"), Hash40::new("toel"), -1, -1, 0, 0, 0, 140, 0.72, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 0.7, 2.5);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
        EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fk_air"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 6, 70, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.55, 0.85, 1.0);
    }
}

unsafe extern "C" fn sound_speciallw_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_captain_003"));
    }
    wait(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_l01"));
    }
}

unsafe extern "C" fn sound_speciallw_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_captain_003"));
    }
    wait(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_l01"));
        PLAY_SE(agent, Hash40::new("se_common_elec_m_damage"));
    }
}

unsafe extern "C" fn sound_specialairlwend_fury(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_special_l01"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_l02"));
    }
}

unsafe extern "C" fn sound_specialairlwend_lightning(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_special_l01"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_l02"));
    }
}

pub fn install(agent: &mut Agent) {
    config::custom_acmd!(agent, Special::Neutral, "game_specialn", 2 => game_specialn_dash, 3 => game_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "game_specialnturn", 2 => game_specialnturn_dash, 3 => game_specialnturn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "game_specialairn", 2 => game_specialairn_dash, 3 => game_specialairn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "game_specialairnturn", 2 => game_specialairnturn_dash, 3 => game_specialairnturn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialn", 2 => effect_specialn_dash, 3 => effect_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialnturn", 2 => effect_specialnturn_dash, 3 => effect_specialnturn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairn", 2 => effect_specialn_dash, 3 => effect_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "effect_specialairnturn", 2 => effect_specialnturn_dash, 3 => effect_specialnturn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialn", 2 => sound_specialn_dash, 3 => sound_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialnturn", 2 => sound_specialnturn_dash, 3 => sound_specialnturn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialairn", 2 => sound_specialn_dash, 3 => sound_specialn_mighty);
    config::custom_acmd!(agent, Special::Neutral, "sound_specialairnturn", 2 => sound_specialnturn_dash, 3 => sound_specialnturn_mighty);
    config::custom_acmd!(agent, Special::Side, "game_specialsstart", 2 => game_specialsstart_heavy, 3 => game_specialsstart_windup);
    config::custom_acmd!(agent, Special::Side, "game_specialairsstart", 2 => game_specialairsstart_heavy, 3 => game_specialairsstart_windup);
    config::custom_acmd!(agent, Special::Side, "game_specialsend", 2 => game_specialsend_heavy);
    config::custom_acmd!(agent, Special::Side, "game_specialairsend", 2 => game_specialairsend_heavy, 3 => game_specialairsend_windup);
    config::custom_acmd!(agent, Special::Side, "effect_specialsstart", 2 => effect_specialsstart_heavy, 3 => effect_specialsstart_windup);
    config::custom_acmd!(agent, Special::Side, "effect_specialairsstart", 2 => effect_specialairsstart_heavy, 3 => effect_specialairsstart_windup);
    config::custom_acmd!(agent, Special::Side, "effect_specialsend", 2 => effect_specialsend_heavy);
    config::custom_acmd!(agent, Special::Side, "effect_specialairsend", 2 => effect_specialairsend_heavy);
    config::custom_acmd!(agent, Special::Side, "sound_specialsstart", 2 => sound_specialsstart_heavy, 3 => sound_specialsstart_windup);
    config::custom_acmd!(agent, Special::Side, "sound_specialairsstart", 2 => sound_specialairsstart_heavy, 3 => sound_specialairsstart_windup);
    config::custom_acmd!(agent, Special::Side, "sound_specialsend", 2 => sound_specialsend_heavy);
    config::custom_acmd!(agent, Special::Side, "sound_specialairsend", 2 => sound_specialsend_heavy);
    config::custom_acmd!(agent, Special::Up, "game_specialhi", 2 => game_specialhi_strike, 3 => game_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "game_specialairhi", 2 => game_specialhi_strike, 3 => game_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "effect_specialhi", 2 => effect_specialhi_strike, 3 => effect_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "effect_specialairhi", 2 => effect_specialairhi_strike, 3 => effect_specialairhi_explosive);
    config::custom_acmd!(agent, Special::Up, "sound_specialhi", 2 => sound_specialhi_strike, 3 => sound_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "sound_specialairhi", 2 => sound_specialhi_strike, 3 => sound_specialhi_explosive);
    config::custom_acmd!(agent, Special::Up, "game_specialhicatch", 3 => game_specialhicatch_explosive);
    config::custom_acmd!(agent, Special::Up, "game_specialhithrow", 3 => game_specialhithrow_explosive);
    config::custom_acmd!(agent, Special::Down, "game_speciallw", 2 => game_speciallw_fury, 3 => game_speciallw_lightning);
    config::custom_acmd!(agent, Special::Down, "game_specialairlw", 2 => game_specialairlw_fury, 3 => game_specialairlw_lightning);
    config::custom_acmd!(agent, Special::Down, "game_specialairlwend", 2 => game_specialairlwend_fury, 3 => game_specialairlwend_lightning);
    config::custom_acmd!(agent, Special::Down, "game_speciallwendair", 2 => game_speciallwendair_fury, 3 => game_speciallwendair_lightning);
    config::custom_acmd!(agent, Special::Down, "game_speciallwend", 3 => game_speciallwend_lightning);
    config::custom_acmd!(agent, Special::Down, "effect_speciallw", 2 => effect_speciallw_fury, 3 => effect_speciallw_lightning);
    config::custom_acmd!(agent, Special::Down, "effect_specialairlw", 2 => effect_specialairlw_fury, 3 => effect_specialairlw_lightning);
    config::custom_acmd!(agent, Special::Down, "sound_speciallw", 2 => sound_speciallw_fury, 3 => sound_speciallw_lightning);
    config::custom_acmd!(agent, Special::Down, "sound_specialairlw", 2 => sound_speciallw_fury, 3 => sound_speciallw_lightning);
    config::custom_acmd!(agent, Special::Down, "sound_specialairlwend", 2 => sound_specialairlwend_fury, 3 => sound_specialairlwend_lightning);
}
