use crate::enums::Shiny;
use crate::gen8::{Den, DenEvent, Raid};
use crate::resources::encounter_data_8::NESTS;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Copy, Clone)]
struct DenInfo {
    hash: [u64; 2],
    location: u8,
    coordinate: [u16; 2],
}

impl DenInfo {
    pub const fn new(hash1: u64, hash2: u64, location: u8, coord1: u16, coord2: u16) -> Self {
        Self {
            hash: [hash1, hash2],
            location,
            coordinate: [coord1, coord2],
        }
    }
}

const DEN_INFO: [DenInfo; 276] = [
    DenInfo::new(0x173f0456dc5dfc52, 0xba83e1671012ebcd, 12, 185, 977), // 16 52
    DenInfo::new(0x17458556dc634333, 0xba8745671015cb90, 12, 125, 1005), // 37 64
    DenInfo::new(0x17458b56dc634d65, 0x450421d99cf882c1, 12, 114, 936), // 31 90
    DenInfo::new(0x17428156dc610690, 0xba805467100fc65f, 12, 311, 998), // 03 51
    DenInfo::new(0x17428856dc611275, 0xba805767100fcb78, 12, 233, 948), // 04 46
    DenInfo::new(0x17458956dc6349ff, 0xba8747671015cef6, 12, 337, 996), // 33 62
    DenInfo::new(0x17459356dc635afd, 0xba8746671015cd43, 12, 209, 976), // 39 65
    DenInfo::new(0x17428356dc6109f6, 0xba805967100fcede, 12, 136, 906), // 01 48
    DenInfo::new(0x17458b56dc634d65, 0x450421d99cf882c1, 12, 252, 905), // 31 90
    DenInfo::new(0x17491656dc666f6d, 0xba83da671012dfe8, 2, 30, 927),   // 26 59
    DenInfo::new(0x17490856dc6657a3, 0x4500a2d99cf5751d, 2, 12, 851),   // 28 79
    DenInfo::new(0x17491656dc666f6d, 0xba83db671012e19b, 2, 12, 861),   // 26 58
    DenInfo::new(0x17428856dc611275, 0x45041fd99cf87f5b, 2, 20, 913),   // 04 92
    DenInfo::new(0x17491656dc666f6d, 0xba83da671012dfe8, 2, 40, 878),   // 26 59
    DenInfo::new(0x17428256dc610843, 0xba805367100fc4ac, 15, 52, 776),  // 02 50
    DenInfo::new(0x17428656dc610f0f, 0xba805867100fcd2b, 15, 68, 652),  // 06 47
    DenInfo::new(0x0000000000000000, 0x0000000000000000, 15, 50, 700),  // Special
    DenInfo::new(0x17428556dc610d5c, 0xba805d67100fd5aa, 4, 217, 913),  // 07 44
    DenInfo::new(0x17459356dc635afd, 0xba8746671015cd43, 4, 158, 705),  // 39 65
    DenInfo::new(0x17458b56dc634d65, 0xba83d8671012dc82, 4, 220, 759),  // 31 61
    DenInfo::new(0x17458b56dc634d65, 0xba83d8671012dc82, 4, 248, 852),  // 31 61
    DenInfo::new(0x17428b56dc61178e, 0xba8a4e67101810b2, 16, 129, 818), // 09 75
    DenInfo::new(0x17428b56dc61178e, 0xba8a4e67101810b2, 16, 131, 735), // 09 75
    DenInfo::new(0x17501a56dc6c94e7, 0xba805d67100fd5aa, 16, 105, 907), // 42 44
    DenInfo::new(0x17428556dc610d5c, 0xba805d67100fd5aa, 16, 50, 909),  // 07 44
    DenInfo::new(0x17428b56dc61178e, 0x450420d99cf8810e, 16, 98, 750),  // 09 91
    DenInfo::new(0x17458756dc634699, 0xba8748671015d0a9, 16, 107, 652), // 35 63
    DenInfo::new(0x17459256dc63594a, 0xba8745671015cb90, 00, 186, 816), // 38 64
    DenInfo::new(0x17428c56dc611941, 0x450420d99cf8810e, 13, 310, 824), // 08 91
    DenInfo::new(0x17501b56dc6c969a, 0xba8a496710180833, 13, 359, 980), // 41 76
    DenInfo::new(0x17501b56dc6c969a, 0xba8a496710180833, 13, 393, 962), // 41 76
    DenInfo::new(0x17428756dc6110c2, 0xba805767100fcb78, 13, 328, 761), // 05 46
    DenInfo::new(0x17428356dc6109f6, 0xba805c67100fd3f7, 13, 352, 765), // 01 43
    DenInfo::new(0x173f0356dc5dfa9f, 0xba805467100fc65f, 7, 443, 895),  // 15 51
    DenInfo::new(0x173f0056dc5df586, 0xba805e67100fd75d, 7, 388, 817),  // 12 45
    DenInfo::new(0x173eff56dc5df3d3, 0xba805a67100fd091, 7, 443, 830),  // 11 49
    DenInfo::new(0x173f0356dc5dfa9f, 0x45009ed99cf56e51, 7, 410, 952),  // 15 83
    DenInfo::new(0x173eff56dc5df3d3, 0x450097d99cf5626c, 7, 447, 815),  // 11 84
    DenInfo::new(0x173efe56dc5df220, 0xba805967100fcede, 11, 393, 781), // 10 48
    DenInfo::new(0x17501b56dc6c969a, 0xba8a496710180833, 11, 314, 755), // 41 76
    DenInfo::new(0x17490756dc6655f0, 0xba83d9671012de35, 11, 440, 658), // 29 60
    DenInfo::new(0x17501b56dc6c969a, 0xba8a496710180833, 11, 281, 717), // 41 76
    DenInfo::new(0x17490756dc6655f0, 0xba83d9671012de35, 11, 440, 703), // 29 60
    DenInfo::new(0x17490756dc6655f0, 0x450425d99cf8898d, 4, 310, 654),  // 29 86
    DenInfo::new(0x173efe56dc5df220, 0xba805967100fcede, 11, 443, 792), // 10 48
    DenInfo::new(0x173f0256dc5df8ec, 0xba805367100fc4ac, 10, 412, 533), // 14 50
    DenInfo::new(0x17458a56dc634bb2, 0xba83d9671012de35, 10, 345, 537), // 30 60
    DenInfo::new(0x17491456dc666c07, 0xba83dd671012e501, 10, 365, 559), // 24 56
    DenInfo::new(0x17501c56dc6c984d, 0xba8746671015cd43, 10, 408, 570), // 40 65
    DenInfo::new(0x17458656dc6344e6, 0x45009dd99cf56c9e, 1, 193, 295),  // 34 82
    DenInfo::new(0x173f0156dc5df739, 0x450424d99cf887da, 1, 274, 321),  // 13 87
    DenInfo::new(0x17428c56dc611941, 0xba805d67100fd5aa, 1, 328, 330),  // 08 44
    DenInfo::new(0x173f0456dc5dfc52, 0xba83e1671012ebcd, 1, 370, 452),  // 16 52
    DenInfo::new(0x17501c56dc6c984d, 0xba8746671015cd43, 1, 224, 282),  // 40 65
    DenInfo::new(0x17428856dc611275, 0x45041fd99cf87f5b, 1, 342, 312),  // 04 92
    DenInfo::new(0x17428756dc6110c2, 0xba805767100fcb78, 1, 340, 269),  // 05 46
    DenInfo::new(0x17458456dc634180, 0xba8748671015d0a9, 1, 305, 323),  // 36 63
    DenInfo::new(0x17491556dc666dba, 0xba83da671012dfe8, 1, 334, 368),  // 27 59
    DenInfo::new(0x17501c56dc6c984d, 0xba874b671015d5c2, 14, 343, 222), // 40 66
    DenInfo::new(0x17428356dc6109f6, 0x45009cd99cf56aeb, 14, 348, 195), // 01 81
    DenInfo::new(0x17428156dc610690, 0xba805467100fc65f, 14, 200, 244), // 03 51
    DenInfo::new(0x173f0756dc5e016b, 0x45009bd99cf56938, 14, 305, 183), // 19 80
    DenInfo::new(0x17428656dc610f0f, 0xba805c67100fd3f7, 14, 348, 180), // 06 43
    DenInfo::new(0x17491556dc666dba, 0xba83da671012dfe8, 14, 260, 199), // 27 59
    DenInfo::new(0x17428256dc610843, 0xba805367100fc4ac, 14, 211, 205), // 02 50
    DenInfo::new(0x173f0056dc5df586, 0x450098d99cf5641f, 14, 303, 242), // 12 85
    DenInfo::new(0x17491256dc6668a1, 0xba83de671012e6b4, 14, 324, 209), // 22 55
    DenInfo::new(0x173f0256dc5df8ec, 0xba805367100fc4ac, 14, 326, 219), // 14 50
    DenInfo::new(0x173f0656dc5dffb8, 0xba83df671012e867, 14, 330, 215), // 18 54
    DenInfo::new(0x17458756dc634699, 0xba8748671015d0a9, 14, 282, 195), // 35 63
    DenInfo::new(0x173f0556dc5dfe05, 0x45041ed99cf87da8, 3, 265, 139),  // 17 93
    DenInfo::new(0x173f0556dc5dfe05, 0xba83e1671012ebcd, 3, 307, 93),   // 17 52
    DenInfo::new(0x173f0356dc5dfa9f, 0x45041ed99cf87da8, 3, 331, 84),   // 15 93
    DenInfo::new(0x17428b56dc61178e, 0xba8a4e67101810b2, 3, 219, 99),   // 09 75
    DenInfo::new(0x173eff56dc5df3d3, 0xba805a67100fd091, 3, 243, 120),  // 11 49
    DenInfo::new(0x173efe56dc5df220, 0xba805967100fcede, 3, 262, 174),  // 10 48
    DenInfo::new(0x17490f56dc666388, 0xba83de671012e6b4, 3, 283, 98),   // 21 55
    DenInfo::new(0x17491056dc66653b, 0xba83df671012e867, 3, 304, 112),  // 20 54
    DenInfo::new(0x17490856dc6657a3, 0xba805c67100fd3f7, 6, 306, 145),  // 28 43
    DenInfo::new(0x17458456dc634180, 0x450423d99cf88627, 3, 214, 168),  // 36 88
    DenInfo::new(0x17501a56dc6c94e7, 0xba874c671015d775, 6, 334, 145),  // 42 67
    DenInfo::new(0x17458456dc634180, 0xba874a671015d40f, 6, 347, 103),  // 36 69
    DenInfo::new(0x17491556dc666dba, 0xba874f671015dc8e, 6, 363, 88),   // 27 70
    DenInfo::new(0x17491356dc666a54, 0xba8a4d6710180eff, 6, 338, 122),  // 25 72
    DenInfo::new(0x173f0056dc5df586, 0xba805e67100fd75d, 8, 339, 35),   // 12 45
    DenInfo::new(0x17458856dc63484c, 0xba83d8671012dc82, 8, 310, 65),   // 32 61
    DenInfo::new(0x17458a56dc634bb2, 0xba83dc671012e34e, 8, 202, 34),   // 30 57
    DenInfo::new(0x173f0756dc5e016b, 0xba83e0671012ea1a, 8, 237, 67),   // 19 53
    DenInfo::new(0x17491156dc6666ee, 0xba8a4c6710180d4c, 8, 183, 47),   // 23 73
    DenInfo::new(0x17458956dc6349ff, 0xba8747671015cef6, 8, 221, 50),   // 33 62
    DenInfo::new(0x173f0256dc5df8ec, 0xba8749671015d25c, 8, 354, 60),   // 14 68
    DenInfo::new(0x17458a56dc634bb2, 0xba83d9671012de35, 5, 181, 185),  // 30 60
    DenInfo::new(0x17491156dc6666ee, 0xba83de671012e6b4, 5, 199, 145),  // 23 55
    DenInfo::new(0x173f0656dc5dffb8, 0xba8750671015de41, 5, 193, 173),  // 18 71
    DenInfo::new(0x17458856dc63484c, 0x450422d99cf88474, 5, 202, 95),   // 32 89
    DenInfo::new(0x17491456dc666c07, 0x4500a1d99cf5736a, 5, 185, 135),  // 24 78
    DenInfo::new(0x17491356dc666a54, 0xba83dd671012e501, 9, 170, 35),   // 25 56
    DenInfo::new(0x173f0656dc5dffb8, 0x4500a0d99cf571b7, 9, 128, 58),   // 18 77
    DenInfo::new(0x17428c56dc611941, 0xba805d67100fd5aa, 9, 161, 80),   // 08 44
    DenInfo::new(0x17458656dc6344e6, 0xba8a4f6710181265, 9, 143, 39),   // 34 74
    DenInfo::new(0x79b25a4f80255a38, 0xc8ea8c1618ab0a58, 17, 643, 822), // 115 116
    DenInfo::new(0xe2c6e5e725342f4a, 0x89955cc3a594e51a, 17, 770, 794), // 125 126
    DenInfo::new(0x6d015b7858eb5119, 0x53441b80e563ef1f, 17, 723, 812), // 109 110
    DenInfo::new(0x4257e50e1c471596, 0xfe9697f9799c65be, 17, 862, 770), // 133 134
    DenInfo::new(0x2998f2424d0353eb, 0xae57b2a84974c3a1, 17, 673, 766), // 111 112
    DenInfo::new(0x5b72bfac0ff3f885, 0x316e6b5e74bc7aa3, 17, 882, 745), // 113 114
    DenInfo::new(0x21f6c965b3513d5e, 0xd8f100cde5822516, 17, 661, 838), // 99  100
    DenInfo::new(0x6e6b46639f77f0c8, 0x1c1962488c012ee8, 17, 683, 792), // 105 106
    DenInfo::new(0xbc3d01fff751cde4, 0x6f948f09933cdfc, 17, 831, 770),  // 123 124
    DenInfo::new(0x4257e30e1c471230, 0xfe9695f9799c6258, 17, 727, 779), // 137 138
    DenInfo::new(0x4257e40e1c4713e3, 0xfe9696f9799c640b, 18, 662, 681), // 135 136
    DenInfo::new(0x2998f2424d0353eb, 0xae57b2a84974c3a1, 18, 741, 680), // 111 112
    DenInfo::new(0xe2c6e5e725342f4a, 0x89955cc3a594e51a, 18, 697, 645), // 125 126
    DenInfo::new(0xc63dec8a65b5c540, 0x6aebee2a2d6d8470, 18, 732, 631), // 121 122
    DenInfo::new(0x5c9a35ca819b38c8, 0xf9222e1acdf486e8, 18, 634, 623), // 119 120
    DenInfo::new(0xb8a5e528bfee71bc, 0xdf017f3fefba2704, 18, 603, 591), // 117 118
    DenInfo::new(0x21f6c865b3513bab, 0xd8f0ffcde5822363, 18, 667, 614), // 101 102
    DenInfo::new(0x21f6c965b3513d5e, 0xd8f100cde5822516, 18, 609, 668), // 99  100
    DenInfo::new(0x4257e50e1c471596, 0xfe9697f9799c65be, 18, 554, 577), // 133 134
    DenInfo::new(0x6d015b7858eb5119, 0x53441b80e563ef1f, 19, 533, 524), // 109 110
    DenInfo::new(0xdb8629cba3383296, 0x4f1e561dd73ed3d8, 19, 687, 535), // 154 145
    DenInfo::new(0x6e6b46639f77f0c8, 0x1c1962488c012ee8, 19, 622, 521), // 105 106
    DenInfo::new(0xe2c6e5e725342f4a, 0x89955cc3a594e51a, 19, 578, 512), // 125 126
    DenInfo::new(0x5c9a35ca819b38c8, 0xf9222e1acdf486e8, 19, 636, 492), // 119 120
    DenInfo::new(0x4257e40e1c4713e3, 0xfe9696f9799c640b, 19, 553, 529), // 135 136
    DenInfo::new(0x5b72bfac0ff3f885, 0x316e6b5e74bc7aa3, 20, 488, 480), // 113 114
    DenInfo::new(0x4257e40e1c4713e3, 0xfe9696f9799c640b, 20, 483, 556), // 135 136
    DenInfo::new(0x6e6b46639f77f0c8, 0x1c1962488c012ee8, 20, 465, 605), // 105 106
    DenInfo::new(0xbc3d01fff751cde4, 0x6f948f09933cdfc, 20, 446, 649),  // 123 124
    DenInfo::new(0x60ef1d711ae30cf0, 0xc80756327d5de060, 20, 453, 561), // 117 118
    DenInfo::new(0x4257e30e1c471230, 0xfe9695f9799c6258, 20, 320, 526), // 137 138
    DenInfo::new(0xb8a5e528bfee71bc, 0xdf017f3fefba2704, 20, 442, 609), // 103 104
    DenInfo::new(0x4c12cee7784c8b8, 0x7288f0346fd3cdd8, 20, 412, 566),  // 127 128
    DenInfo::new(0x50eaf4685fa07085, 0xf9280759d6cc62a3, 21, 947, 506), // 129 130
    DenInfo::new(0xbc3d01fff751cde4, 0x6f948f09933cdfc, 22, 912, 467),  // 123 124
    DenInfo::new(0x5584521f1e549486, 0x55846e1f1e54c41a, 22, 925, 433), // 156 157
    DenInfo::new(0xa178d4769765abac, 0xf4a830850f51d034, 22, 913, 408), // 107 108
    DenInfo::new(0xc63dec8a65b5c540, 0x6aebee2a2d6d8470, 22, 895, 365), // 121 122
    DenInfo::new(0x60ef1d711ae30cf0, 0xc80756327d5de060, 23, 526, 650), // 103 104
    DenInfo::new(0x4257e40e1c4713e3, 0x4f1e5b1dd73edc57, 23, 576, 714), // 135 148
    DenInfo::new(0x4c12cee7784c8b8, 0x7288f0346fd3cdd8, 23, 565, 726),  // 127 128
    DenInfo::new(0x50eaf4685fa07085, 0xf9280759d6cc62a3, 23, 586, 726), // 129 130
    DenInfo::new(0x4257e50e1c471596, 0xfe9697f9799c65be, 23, 621, 749), // 133 134
    DenInfo::new(0x21f6c865b3513bab, 0xd8f0ffcde5822363, 23, 528, 695), // 101 102
    DenInfo::new(0x5b72bfac0ff3f885, 0x316e6b5e74bc7aa3, 24, 408, 809), // 113 114
    DenInfo::new(0xb0c9af2202b0a19e, 0x4f1e5c1dd73ede0a, 24, 426, 790), // 131 151
    DenInfo::new(0x4257e30e1c471230, 0xfe9695f9799c6258, 24, 360, 850), // 137 138
    DenInfo::new(0xa178d4769765abac, 0xf4a830850f51d034, 24, 327, 787), // 107 108
    DenInfo::new(0x5c9a35ca819b38c8, 0xf9222e1acdf486e8, 25, 707, 421), // 119 120
    DenInfo::new(0xdb8629cba3383296, 0x4f1e561dd73ed3d8, 25, 832, 398), // 154 145
    DenInfo::new(0xb8a5e528bfee71bc, 0xdf017f3fefba2704, 25, 591, 430), // 117 118
    DenInfo::new(0x79b25a4f80255a38, 0xc8ea8c1618ab0a58, 25, 666, 334), // 115 116
    DenInfo::new(0x2998f2424d0353eb, 0xae57b2a84974c3a1, 25, 758, 338), // 111 112
    DenInfo::new(0x6d015b7858eb5119, 0x4f1e5a1dd73edaa4, 25, 719, 377), // 109 149
    DenInfo::new(0x21f6c865b3513bab, 0xd8f0ffcde5822363, 25, 659, 397), // 101 102
    DenInfo::new(0x60ef1d711ae30cf0, 0x4f1e5d1dd73edfbd, 26, 665, 243), // 103 150
    DenInfo::new(0x5b72bfac0ff3f885, 0x316e6b5e74bc7aa3, 26, 784, 212), // 113 114
    DenInfo::new(0x79b25a4f80255a38, 0xc8ea8c1618ab0a58, 26, 881, 235), // 115 116
    DenInfo::new(0x6b37a94863bf68c0, 0x4f1e591dd73ed8f1, 27, 321, 1004), // 155 146
    DenInfo::new(0x4257ea0e1c471e15, 0xfe969cf9799c6e3d, 27, 782, 962), // 139 140
    DenInfo::new(0x40bdbe4f3bcbac86, 0x9fdf11a0cde96b2e, 27, 1040, 752), // 152 153
    DenInfo::new(0xc63dec8a65b5c540, 0x6aebee2a2d6d8470, 27, 970, 701), // 121 122
    DenInfo::new(0x6d015b7858eb5119, 0x53441b80e563ef1f, 27, 759, 1015), // 109 110
    DenInfo::new(0xa178d4769765abac, 0xf4a830850f51d034, 27, 558, 1082), // 107 108
    DenInfo::new(0xb0c9af2202b0a19e, 0x3d6f1fcb3898d356, 27, 523, 993), // 131 132
    DenInfo::new(0x60ef1d711ae30cf0, 0xc80756327d5de060, 28, 129, 797), // 103 104
    DenInfo::new(0xb8a5e528bfee71bc, 0xdf017f3fefba2704, 28, 75, 658),  // 117 118
    DenInfo::new(0x6b37a94863bf68c0, 0x4f1e591dd73ed8f1, 28, 120, 523), // 155 146
    DenInfo::new(0x5c9a35ca819b38c8, 0xf9222e1acdf486e8, 28, 120, 547), // 119 120
    DenInfo::new(0x50eaf4685fa07085, 0xf9280759d6cc62a3, 28, 287, 559), // 129 130
    DenInfo::new(0x21f6c965b3513d5e, 0xd8f100cde5822516, 28, 258, 654), // 99  100
    DenInfo::new(0x4c12cee7784c8b8, 0x7288f0346fd3cdd8, 28, 174, 852),  // 127 128
    DenInfo::new(0x4257ea0e1c471e15, 0xfe969cf9799c6e3d, 28, 162, 808), // 139 140
    DenInfo::new(0xa178d4769765abac, 0xf4a830850f51d034, 28, 162, 763), // 107 108
    DenInfo::new(0xe2c6e5e725342f4a, 0x89955cc3a594e51a, 29, 299, 356), // 125 126
    DenInfo::new(0x21f6c965b3513d5e, 0xd8f100cde5822516, 29, 214, 349), // 99  100
    DenInfo::new(0x2998f2424d0353eb, 0xae57b2a84974c3a1, 29, 185, 302), // 111 112
    DenInfo::new(0x4257ea0e1c471e15, 0xfe969cf9799c6e3d, 29, 247, 298), // 139 140
    DenInfo::new(0x4257e30e1c471230, 0x4f1e581dd73ed73e, 29, 271, 273), // 137 147
    DenInfo::new(0xb0c9af2202b0a19e, 0x3d6f1fcb3898d356, 30, 468, 451), // 131 132
    DenInfo::new(0xbc3d01fff751cde4, 0x6f948f09933cdfc, 30, 605, 166),  // 123 124
    DenInfo::new(0xc63dec8a65b5c540, 0x6aebee2a2d6d8470, 30, 672, 120), // 121 122
    DenInfo::new(0x4257ea0e1c471e15, 0xfe969cf9799c6e3d, 30, 716, 91),  // 139 140
    DenInfo::new(0x6e6b46639f77f0c8, 0x1c1962488c012ee8, 30, 597, 105), // 105 106
    DenInfo::new(0xea4c3915ea6f95a0, 0x3ea9df3b7b2b5990, 31, 471, 152), // 143 144
    DenInfo::new(0xea4c3915ea6f95a0, 0x3ea9df3b7b2b5990, 31, 490, 194), // 143 144
    DenInfo::new(0xea4c3915ea6f95a0, 0x3ea9df3b7b2b5990, 31, 464, 237), // 143 144
    DenInfo::new(0xea4c3915ea6f95a0, 0x3ea9df3b7b2b5990, 31, 413, 237), // 143 144
    DenInfo::new(0xea4c3915ea6f95a0, 0x3ea9df3b7b2b5990, 31, 386, 195), // 143 144
    DenInfo::new(0xea4c3915ea6f95a0, 0x3ea9df3b7b2b5990, 31, 414, 148), // 143 144
    DenInfo::new(0x779e9eb99c1292c, 0x93a637943a964e41, 32, 0, 0),      // 166 167
    DenInfo::new(0x55e4467f01ec60bb, 0xa5696e4aa8d625a, 32, 0, 0),      // 180 181
    DenInfo::new(0x685db02aaedbcf61, 0x2cd8cf9a88739f98, 32, 0, 0),     // 168 169
    DenInfo::new(0x2640fa844b19c3cf, 0x422f95fb66a95706, 32, 0, 0),     // 158 159
    DenInfo::new(0x47a5d8b98dd573ab, 0xa23ec426e4e9430a, 32, 0, 0),     // 182 183
    DenInfo::new(0x685db02aaedbcf61, 0x2cd8cf9a88739f98, 32, 0, 0),     // 168 169
    DenInfo::new(0xc862667fc72ee059, 0x72f9d87337338120, 33, 0, 0),     // 190 191
    DenInfo::new(0x42b21efc37c7b974, 0x9d415f6a7a841dd9, 33, 0, 0),     // 176 177
    DenInfo::new(0x9ab5727f28c3d593, 0x1928030ad989ad02, 33, 0, 0),     // 186 187
    DenInfo::new(0x75319113c8c3b924, 0x314acb827c75109, 33, 0, 0),      // 172 173
    DenInfo::new(0xe234e939402a736b, 0x3b3c0865d15b0aca, 33, 0, 0),     // 164 165
    DenInfo::new(0x7ea57d4a1ef4c796, 0xe0236c3b91edbebb, 34, 0, 0),     // 188 189
    DenInfo::new(0x3a41c5c485d3edee, 0x6c364ecc3616af63, 34, 0, 0),     // 160 161
    DenInfo::new(0x52a7dfe87897d15d, 0xc88b8a5990a8ea5c, 34, 0, 0),     // 174 175
    DenInfo::new(0xf01dfb231a467c06, 0x8b5a3178ae3f236b, 34, 0, 0),     // 192 193
    DenInfo::new(0x2640fa844b19c3cf, 0x422f95fb66a95706, 34, 0, 0),     // 158 159
    DenInfo::new(0xf6389ad0bc9aaeb, 0x277effbe0b116e4a, 34, 0, 0),      // 162 163
    DenInfo::new(0x3d2f6b02fc6dd797, 0xf9d3242b837d627e, 34, 0, 0),     // 184 185
    DenInfo::new(0x75319113c8c3b924, 0x314acb827c75109, 34, 0, 0),      // 172 173
    DenInfo::new(0x779e9eb99c1292c, 0x93a637943a964e41, 34, 0, 0),      // 166 167
    DenInfo::new(0x55e4467f01ec60bb, 0xa5696e4aa8d625a, 34, 0, 0),      // 180 181
    DenInfo::new(0x52a7dfe87897d15d, 0xc88b8a5990a8ea5c, 34, 0, 0),     // 174 175
    DenInfo::new(0x9ab5727f28c3d593, 0x1928030ad989ad02, 34, 0, 0),     // 186 187
    DenInfo::new(0xc862667fc72ee059, 0x72f9d87337338120, 34, 0, 0),     // 190 191
    DenInfo::new(0x55e4467f01ec60bb, 0xa5696e4aa8d625a, 34, 0, 0),      // 180 181
    DenInfo::new(0x685dad2aaedbca48, 0x12ad4e9a799417a5, 34, 0, 0),     // 170 171
    DenInfo::new(0xf6389ad0bc9aaeb, 0x277effbe0b116e4a, 34, 0, 0),      // 162 163
    DenInfo::new(0x685dad2aaedbca48, 0x12ad4e9a799417a5, 34, 0, 0),     // 170 171
    DenInfo::new(0x17d327792698d15f, 0xb20a5ed251cd0456, 34, 0, 0),     // 178 179
    DenInfo::new(0xe234e939402a736b, 0x3b3c0865d15b0aca, 34, 0, 0),     // 164 165
    DenInfo::new(0x3a41c5c485d3edee, 0x6c364ecc3616af63, 34, 0, 0),     // 160 161
    DenInfo::new(0x42b21efc37c7b974, 0x9d415f6a7a841dd9, 34, 0, 0),     // 176 177
    DenInfo::new(0x47a5d8b98dd573ab, 0xa23ec426e4e9430a, 35, 0, 0),     // 182 183
    DenInfo::new(0xf01dfb231a467c06, 0x8b5a3178ae3f236b, 35, 0, 0),     // 192 193
    DenInfo::new(0x55e4467f01ec60bb, 0xa5696e4aa8d625a, 36, 0, 0),      // 180 181
    DenInfo::new(0x75319113c8c3b924, 0x314acb827c75109, 36, 0, 0),      // 172 173
    DenInfo::new(0x7ea57d4a1ef4c796, 0xe0236c3b91edbebb, 36, 0, 0),     // 188 189
    DenInfo::new(0x17d327792698d15f, 0xb20a5ed251cd0456, 36, 0, 0),     // 178 179
    DenInfo::new(0x9ab5727f28c3d593, 0x1928030ad989ad02, 36, 0, 0),     // 186 187
    DenInfo::new(0x685db02aaedbcf61, 0x2cd8cf9a88739f98, 36, 0, 0),     // 168 169
    DenInfo::new(0xc862667fc72ee059, 0x72f9d87337338120, 36, 0, 0),     // 190 191
    DenInfo::new(0x3a41c5c485d3edee, 0x6c364ecc3616af63, 36, 0, 0),     // 160 161
    DenInfo::new(0x58c3011eda59ea53, 0xb4dbd8428706d1c2, 36, 0, 0),     // 196 197
    DenInfo::new(0x7ea57d4a1ef4c796, 0xe0236c3b91edbebb, 37, 0, 0),     // 188 189
    DenInfo::new(0x3d2f6b02fc6dd797, 0xf9d3242b837d627e, 37, 0, 0),     // 184 185
    DenInfo::new(0x17d327792698d15f, 0xb20a5ed251cd0456, 37, 0, 0),     // 178 179
    DenInfo::new(0x779e9eb99c1292c, 0x93a637943a964e41, 38, 0, 0),      // 166 167
    DenInfo::new(0x2640fa844b19c3cf, 0x422f95fb66a95706, 39, 0, 0),     // 158 159
    DenInfo::new(0xf6389ad0bc9aaeb, 0x277effbe0b116e4a, 39, 0, 0),      // 162 163
    DenInfo::new(0x9ab5727f28c3d593, 0x1928030ad989ad02, 39, 0, 0),     // 186 187
    DenInfo::new(0x779e9eb99c1292c, 0x93a637943a964e41, 39, 0, 0),      // 166 167
    DenInfo::new(0x47a5d8b98dd573ab, 0xa23ec426e4e9430a, 39, 0, 0),     // 182 183
    DenInfo::new(0x685dad2aaedbca48, 0x12ad4e9a799417a5, 40, 0, 0),     // 170 171
    DenInfo::new(0x3d2f6b02fc6dd797, 0xf9d3242b837d627e, 40, 0, 0),     // 184 185
    DenInfo::new(0xf6389ad0bc9aaeb, 0x277effbe0b116e4a, 40, 0, 0),      // 162 163
    DenInfo::new(0x75319113c8c3b924, 0x314acb827c75109, 40, 0, 0),      // 172 173
    DenInfo::new(0x7ea57d4a1ef4c796, 0xe0236c3b91edbebb, 40, 0, 0),     // 188 189
    DenInfo::new(0x685dad2aaedbca48, 0x12ad4e9a799417a5, 40, 0, 0),     // 170 171
    DenInfo::new(0x52a7dfe87897d15d, 0xc88b8a5990a8ea5c, 40, 0, 0),     // 174 175
    DenInfo::new(0x47a5d8b98dd573ab, 0xa23ec426e4e9430a, 40, 0, 0),     // 182 183
    DenInfo::new(0x9ab5727f28c3d593, 0x1928030ad989ad02, 40, 0, 0),     // 186 187
    DenInfo::new(0xc862667fc72ee059, 0x72f9d87337338120, 40, 0, 0),     // 190 191
    DenInfo::new(0x685db02aaedbcf61, 0x2cd8cf9a88739f98, 40, 0, 0),     // 168 169
    DenInfo::new(0xe234e939402a736b, 0x3b3c0865d15b0aca, 40, 0, 0),     // 164 165
    DenInfo::new(0x42b21efc37c7b974, 0x9d415f6a7a841dd9, 40, 0, 0),     // 176 177
    DenInfo::new(0x685dad2aaedbca48, 0x12ad4e9a799417a5, 40, 0, 0),     // 170 171
    DenInfo::new(0x3d2f6b02fc6dd797, 0xf9d3242b837d627e, 41, 0, 0),     // 184 185
    DenInfo::new(0xe234e939402a736b, 0x3b3c0865d15b0aca, 41, 0, 0),     // 164 165
    DenInfo::new(0x2640fa844b19c3cf, 0x422f95fb66a95706, 42, 0, 0),     // 158 159
    DenInfo::new(0x42b21efc37c7b974, 0x9d415f6a7a841dd9, 42, 0, 0),     // 176 177
    DenInfo::new(0x52a7dfe87897d15d, 0xc88b8a5990a8ea5c, 42, 0, 0),     // 174 175
    DenInfo::new(0x55e4467f01ec60bb, 0xa5696e4aa8d625a, 42, 0, 0),      // 180 181
    DenInfo::new(0x3a41c5c485d3edee, 0x6c364ecc3616af63, 42, 0, 0),     // 160 161
    DenInfo::new(0x3a41c5c485d3edee, 0x6c364ecc3616af63, 42, 0, 0),     // 160 161
    DenInfo::new(0x779e9eb99c1292c, 0x93a637943a964e41, 42, 0, 0),      // 166 167
    DenInfo::new(0x52a7dfe87897d15d, 0xc88b8a5990a8ea5c, 42, 0, 0),     // 174 175
    DenInfo::new(0xe234e939402a736b, 0x3b3c0865d15b0aca, 42, 0, 0),     // 164 165
    DenInfo::new(0x75319113c8c3b924, 0x314acb827c75109, 42, 0, 0),      // 172 173
    DenInfo::new(0xe78d0a25d0c67a32, 0xbdf065bb6332909f, 42, 0, 0),     // 194 195
    DenInfo::new(0xf01dfb231a467c06, 0x8b5a3178ae3f236b, 42, 0, 0),     // 192 193
    DenInfo::new(0xc862667fc72ee059, 0x72f9d87337338120, 42, 0, 0),     // 190 191
    DenInfo::new(0xf6389ad0bc9aaeb, 0x277effbe0b116e4a, 42, 0, 0),      // 162 163
    DenInfo::new(0x17d327792698d15f, 0xb20a5ed251cd0456, 42, 0, 0),     // 178 179
    DenInfo::new(0x47a5d8b98dd573ab, 0xa23ec426e4e9430a, 42, 0, 0),     // 182 183
    DenInfo::new(0x3d2f6b02fc6dd797, 0xf9d3242b837d627e, 42, 0, 0),     // 184 185
    DenInfo::new(0x42b21efc37c7b974, 0x9d415f6a7a841dd9, 43, 0, 0),     // 176 177
];

static EVENT: Mutex<DenEvent> = Mutex::new(DenEvent::blank());

#[derive(Deserialize)]
struct EventData {
    #[serde(rename = "Tables")]
    tables: Vec<EventTable>,
}

#[derive(Deserialize)]
struct EventTable {
    #[serde(rename = "Entries")]
    entries: Vec<EventTableEntry>,
}

#[derive(Deserialize, Copy, Clone)]
struct EventTableEntry {
    #[serde(rename = "Ability")]
    ability: u8,
    #[serde(rename = "AltForm")]
    alt_form: u8,
    #[serde(rename = "ShinyForced")]
    shiny_forced: u8,
    #[serde(rename = "FlawlessIVs")]
    flawless_ivs: u8,
    #[serde(rename = "Gender")]
    gender: u8,
    #[serde(rename = "IsGigantamax")]
    is_gigantamax: bool,
    #[serde(rename = "Species")]
    species: u16,
    #[serde(rename = "Probabilities")]
    probabilities: [u8; 5],
}

impl From<EventTableEntry> for Raid {
    fn from(value: EventTableEntry) -> Self {
        let mut stars = [false; 5];
        for (i, probability) in value.probabilities.into_iter().enumerate() {
            stars[i] = probability > 0;
        }
        Raid::new(
            value.species,
            value.alt_form,
            Shiny::from(value.shiny_forced),
            value.ability,
            value.gender,
            value.flawless_ivs,
            value.is_gigantamax,
            stars,
        )
    }
}

/// Initialize the DenLoader with what `path` to use for events.
///
/// This must be called before using [`get_event()`]
pub fn init(mut path: PathBuf) {
    path.push("nests_event.json");
    let read = OpenOptions::new().read(true).open(path);
    if let Ok(read) = read {
        let reader = BufReader::new(read);
        let event_data = serde_json::from_reader::<_, EventData>(reader);
        if let Ok(event_data) = event_data {
            let sword_entries = &event_data.tables[0].entries;
            let shield_entries = &event_data.tables[1].entries;

            let sword_raids = sword_entries
                .iter()
                .copied()
                .filter_map(|e| {
                    let raid: Raid = e.into();
                    if raid.star.iter().any(|&b| b) {
                        Some(raid)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let shield_raids = shield_entries
                .iter()
                .copied()
                .filter_map(|e| {
                    let raid: Raid = e.into();
                    if raid.star.iter().any(|&b| b) {
                        Some(raid)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            *EVENT.lock().unwrap() = DenEvent::new(sword_raids, shield_raids);
        }
    }
}

/// Returns the den for the `index` and `rarity`
pub fn get_den(index: usize, rarity: usize) -> &'static Den {
    let table_hash = DEN_INFO[index].hash[rarity];
    NESTS.iter().find(|d| d.hash >= table_hash).unwrap()
}

/// Returns the den event initialized by [`init()`]
pub fn get_event() -> &'static Mutex<DenEvent> {
    &EVENT
}

/// Returns the location of the den
pub fn get_location(index: usize) -> u8 {
    DEN_INFO[index].location
}

/// Returns the coordinates of the den
pub fn get_coordinate(index: usize) -> [u16; 2] {
    DEN_INFO[index].coordinate
}
