use crate::base_printer::position::RawPosition;

pub(crate) enum AreaTrigger {
    Circle {
        position: RawPosition,
        radius: f32,
    },
    Square {
        position: RawPosition,
        /// Size along the x axis.
        length: f32,
        /// Size along the y axis.
        width: f32,
        /// Size along the z axis.
        height: f32,
        /// Rotation about the Z axis
        yaw: f32,
    },
}

impl AreaTrigger {
    pub const fn position(&self) -> &RawPosition {
        match self {
            AreaTrigger::Circle { position, .. } => position,
            AreaTrigger::Square { position, .. } => position,
        }
    }
}

pub(crate) const AREA_TRIGGERS: &[(u32, AreaTrigger)] = &[
    (
        45,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2924.4, -798.4, 161.6),
            radius: 8.0,
        },
    ),
    (
        60,
        AreaTrigger::Circle {
            position: RawPosition::no_names(24, 47.6, -28.9, -8.8),
            radius: 7.0,
        },
    ),
    (
        71,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -10645.9, 1179.1, 48.2),
            radius: 27.0,
        },
    ),
    (
        78,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11208.5, 1685.3, 25.8),
            radius: 7.0,
        },
    ),
    (
        81,
        AreaTrigger::Circle {
            position: RawPosition::no_names(28, -259.2, 137.8, 36.6),
            radius: 7.0,
        },
    ),
    (
        83,
        AreaTrigger::Circle {
            position: RawPosition::no_names(28, 254.6, -4.3, 6.5),
            radius: 8.0,
        },
    ),
    (
        84,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 16449.9, 16393.2, 69.4),
            radius: 15.0,
        },
    ),
    (
        87,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9077.3, -552.9, 60.3),
            radius: 30.0,
        },
    ),
    (
        88,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9843.5, 127.5, 5.4),
            radius: 10.0,
        },
    ),
    (
        95,
        AreaTrigger::Circle {
            position: RawPosition::no_names(30, -409.8, -802.8, -13.2),
            radius: 9.0,
        },
    ),
    (
        97,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5660.3, 755.3, 389.6),
            radius: 15.0,
        },
    ),
    (
        98,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11634.8, -54.1, 13.4),
            radius: 30.0,
        },
    ),
    (
        100,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2019.3, 1904.4, 105.1),
            radius: 7.0,
        },
    ),
    (
        101,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -8761.8, 848.6, 87.8),
            length: 5.0,
            width: 9.7,
            height: 7.4,
            yaw: 0.7,
        },
    ),
    (
        107,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8667.6, 623.6, 85.4),
            radius: 7.0,
        },
    ),
    (
        109,
        AreaTrigger::Circle {
            position: RawPosition::no_names(35, -0.9, 16.4, -14.2),
            radius: 7.0,
        },
    ),
    (
        119,
        AreaTrigger::Circle {
            position: RawPosition::no_names(36, -14.4, -393.4, 64.6),
            radius: 6.0,
        },
    ),
    (
        121,
        AreaTrigger::Circle {
            position: RawPosition::no_names(36, -110.8, -1009.2, 34.8),
            radius: 7.0,
        },
    ),
    (
        145,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -229.5, 1576.3, 78.9),
            radius: 7.0,
        },
    ),
    (
        168,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5584.2, 759.8, 384.3),
            radius: 15.0,
        },
    ),
    (
        169,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5586.5, 679.3, 385.0),
            radius: 8.0,
        },
    ),
    (
        171,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5763.7, -3439.6, 306.4),
            radius: 50.0,
        },
    ),
    (
        173,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1058.1, 1554.7, -15.0),
            radius: 90.0,
        },
    ),
    (
        175,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -4086.4, -2610.9, 46.0),
            radius: 20.0,
        },
    ),
    (
        178,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 683.8, -912.1, 174.5),
            radius: 5.0,
        },
    ),
    (
        194,
        AreaTrigger::Circle {
            position: RawPosition::no_names(33, -231.0, 2105.1, 79.8),
            radius: 5.0,
        },
    ),
    (
        196,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -12153.6, 894.0, 9.5),
            radius: 80.0,
        },
    ),
    (
        197,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9796.2, 157.8, 25.4),
            radius: 9.0,
        },
    ),
    (
        205,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -1895.8, -1109.0, 86.2),
            radius: 15.0,
        },
    ),
    (
        216,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 90.1, -1943.4, 79.5),
            radius: 9.0,
        },
    ),
    (
        217,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 10708.8, 762.1, 1321.4),
            radius: 14.0,
        },
    ),
    (
        218,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 9859.1, 588.8, 1300.6),
            radius: 14.0,
        },
    ),
    (
        219,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 9556.2, 1654.8, 1299.5),
            radius: 14.0,
        },
    ),
    (
        220,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 10675.2, 1857.4, 1324.2),
            radius: 10.0,
        },
    ),
    (
        223,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4508.2, 373.4, 32.5),
            radius: 40.0,
        },
    ),
    (
        224,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4605.6, 385.2, 31.5),
            radius: 25.0,
        },
    ),
    (
        225,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4558.4, 421.4, 33.7),
            radius: 50.0,
        },
    ),
    (
        226,
        AreaTrigger::Circle {
            position: RawPosition::no_names(43, -172.2, 139.0, -66.6),
            radius: 12.0,
        },
    ),
    (
        228,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -753.6, -2212.8, 21.5),
            radius: 13.0,
        },
    ),
    (
        230,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6745.0, 41.1, 47.6),
            radius: 13.0,
        },
    ),
    (
        231,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5996.6, 365.2, 21.9),
            radius: 40.0,
        },
    ),
    (
        232,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5980.2, 330.7, 21.8),
            radius: 40.0,
        },
    ),
    (
        233,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4917.0, 328.4, 36.8),
            radius: 50.0,
        },
    ),
    (
        234,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4641.2, 55.4, 66.6),
            radius: 60.0,
        },
    ),
    (
        235,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4554.6, 146.4, 59.5),
            radius: 70.0,
        },
    ),
    (
        236,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4643.4, 145.3, 57.9),
            radius: 50.0,
        },
    ),
    (
        237,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4504.3, 32.3, 86.5),
            radius: 80.0,
        },
    ),
    (
        238,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6876.3, -486.0, 40.1),
            radius: 65.0,
        },
    ),
    (
        239,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 3469.4, 847.6, 5.4),
            radius: 12.0,
        },
    ),
    (
        242,
        AreaTrigger::Circle {
            position: RawPosition::no_names(47, 1936.4, 1534.5, 86.9),
            radius: 8.0,
        },
    ),
    (
        244,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4456.7, -1656.0, 86.1),
            radius: 8.0,
        },
    ),
    (
        246,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4932.5, -1596.1, 84.8),
            radius: 20.0,
        },
    ),
    (
        254,
        AreaTrigger::Circle {
            position: RawPosition::no_names(33, -236.5, 2202.4, 97.3),
            radius: 5.0,
        },
    ),
    (
        255,
        AreaTrigger::Circle {
            position: RawPosition::no_names(33, -223.4, 2102.0, 97.4),
            radius: 5.0,
        },
    ),
    (
        256,
        AreaTrigger::Circle {
            position: RawPosition::no_names(33, -189.0, 2145.7, 97.4),
            radius: 7.0,
        },
    ),
    (
        257,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4252.4, 757.0, -23.1),
            radius: 12.0,
        },
    ),
    (
        259,
        AreaTrigger::Circle {
            position: RawPosition::no_names(48, -148.6, 121.1, -33.4),
            radius: 12.0,
        },
    ),
    (
        262,
        AreaTrigger::Circle {
            position: RawPosition::no_names(47, 2019.7, 2004.6, 61.7),
            radius: 15.0,
        },
    ),
    (
        283,
        AreaTrigger::Circle {
            position: RawPosition::no_names(48, -527.7, 321.1, -50.3),
            radius: 20.0,
        },
    ),
    (
        284,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8681.4, 434.4, 99.3),
            radius: 7.0,
        },
    ),
    (
        286,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -6053.7, -2954.6, 213.7),
            radius: 10.0,
        },
    ),
    (
        288,
        AreaTrigger::Circle {
            position: RawPosition::no_names(70, -228.2, 34.2, -39.2),
            radius: 10.0,
        },
    ),
    (
        302,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -3463.3, -4123.1, 17.1),
            radius: 8.0,
        },
    ),
    (
        303,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4058.8, -2732.5, 35.3),
            radius: 10.0,
        },
    ),
    (
        322,
        AreaTrigger::Circle {
            position: RawPosition::no_names(90, -312.2, -4.6, -148.7),
            radius: 14.0,
        },
    ),
    (
        324,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5161.3, 939.6, 258.3),
            radius: 10.0,
        },
    ),
    (
        342,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9029.2, -608.3, 56.5),
            radius: 30.0,
        },
    ),
    (
        362,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -10429.4, -3828.8, -31.6),
            radius: 150.0,
        },
    ),
    (
        382,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8681.4, 434.4, 99.3),
            radius: 7.0,
        },
    ),
    (
        422,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -93.2, 1691.2, 89.1),
            radius: 20.0,
        },
    ),
    (
        442,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4666.5, -2536.8, 87.0),
            radius: 10.0,
        },
    ),
    (
        444,
        AreaTrigger::Circle {
            position: RawPosition::no_names(129, 2593.9, 1124.6, 56.2),
            radius: 11.0,
        },
    ),
    (
        446,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -10162.7, -3998.6, -108.0),
            radius: 11.0,
        },
    ),
    (
        448,
        AreaTrigger::Circle {
            position: RawPosition::no_names(109, -300.0, 99.0, -127.0),
            radius: 13.0,
        },
    ),
    (
        462,
        AreaTrigger::Circle {
            position: RawPosition::no_names(47, 2107.7, 1823.8, 80.0),
            radius: 10.0,
        },
    ),
    (
        463,
        AreaTrigger::Circle {
            position: RawPosition::no_names(47, 2165.8, 1944.8, 61.5),
            radius: 15.0,
        },
    ),
    (
        482,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8932.8, -1986.4, 139.9),
            radius: 12.0,
        },
    ),
    (
        503,
        AreaTrigger::Circle {
            position: RawPosition::no_names(34, 39.4, 0.8, -12.8),
            radius: 8.0,
        },
    ),
    (
        522,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -1679.3, -4329.0, 2.6),
            radius: 4.0,
        },
    ),
    (
        523,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -4858.3, 785.0, 241.1),
            radius: 12.0,
        },
    ),
    (
        525,
        AreaTrigger::Circle {
            position: RawPosition::no_names(90, -717.9, 1.4, -241.3),
            radius: 12.0,
        },
    ),
    (
        527,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 9947.5, 2630.0, 1318.6),
            radius: 10.0,
        },
    ),
    (
        542,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 8799.4, 969.8, 30.2),
            radius: 10.0,
        },
    ),
    (
        562,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9465.6, 16.8, 65.9),
            radius: 30.0,
        },
    ),
    (
        602,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 1687.6, 1040.1, 21.0),
            radius: 8.0,
        },
    ),
    (
        604,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 853.7, 1308.1, 19.7),
            radius: 8.0,
        },
    ),
    (
        606,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 1608.7, -309.0, 20.8),
            radius: 8.0,
        },
    ),
    (
        608,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 253.6, -196.8, 21.0),
            radius: 8.0,
        },
    ),
    (
        610,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2925.2, -820.5, 161.6),
            radius: 8.0,
        },
    ),
    (
        612,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2878.0, -839.3, 163.0),
            radius: 6.0,
        },
    ),
    (
        614,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2859.7, -824.9, 162.1),
            radius: 8.0,
        },
    ),
    (
        682,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9219.4, -2149.9, 70.6),
            radius: 30.0,
        },
    ),
    (
        702,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9023.3, 892.4, 33.5),
            radius: 7.0,
        },
    ),
    (
        704,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9020.1, 880.5, 151.6),
            radius: 7.0,
        },
    ),
    (
        707,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -10517.0, -1158.4, 39.1),
            radius: 30.0,
        },
    ),
    (
        708,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -854.5, -576.3, 18.5),
            length: 56.2,
            width: 27.5,
            height: 28.5,
            yaw: 4.7,
        },
    ),
    (
        709,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -3615.5, -4467.3, 24.3),
            radius: 30.0,
        },
    ),
    (
        710,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5601.5, -530.7, 395.5),
            radius: 35.0,
        },
    ),
    (
        712,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5390.2, -2953.9, 322.0),
            radius: 36.0,
        },
    ),
    (
        713,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -3823.1, -834.5, 18.3),
            radius: 30.0,
        },
    ),
    (
        715,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 9809.0, 959.2, 1315.3),
            radius: 32.0,
        },
    ),
    (
        716,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6410.0, 527.0, 14.1),
            radius: 32.0,
        },
    ),
    (
        717,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 2756.6, -423.1, 119.4),
            radius: 32.0,
        },
    ),
    (
        719,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2266.7, 246.0, 44.4),
            radius: 30.0,
        },
    ),
    (
        720,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 511.5, 1638.6, 75.4),
            radius: 50.0,
        },
    ),
    (
        721,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -4.9, -934.9, 62.5),
            length: 24.6,
            width: 25.7,
            height: 11.6,
            yaw: 5.8,
        },
    ),
    (
        722,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -2366.7, -346.0, -1.3),
            radius: 30.0,
        },
    ),
    (
        742,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -405.3, -2645.3, 112.0),
            radius: 30.0,
        },
    ),
    (
        743,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -1051.4, -3653.8, 31.1),
            radius: 30.0,
        },
    ),
    (
        762,
        AreaTrigger::Circle {
            position: RawPosition::no_names(48, -842.1, -472.5, -34.1),
            radius: 10.0,
        },
    ),
    (
        802,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -3752.8, -851.6, 10.1),
            radius: 20.0,
        },
    ),
    (
        803,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -3668.3, -823.8, 9.9),
            radius: 12.0,
        },
    ),
    (
        822,
        AreaTrigger::Circle {
            position: RawPosition::no_names(70, -234.4, 319.1, -47.6),
            radius: 20.0,
        },
    ),
    (
        843,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 341.4, -4684.7, 30.9),
            radius: 30.0,
        },
    ),
    (
        844,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -10487.3, -3256.9, 39.9),
            radius: 30.0,
        },
    ),
    (
        862,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -14457.0, 496.5, 39.1),
            radius: 28.0,
        },
    ),
    (
        882,
        AreaTrigger::Circle {
            position: RawPosition::no_names(70, -213.7, 370.6, -35.4),
            radius: 10.0,
        },
    ),
    (
        902,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -6606.5, -3762.2, 266.9),
            radius: 12.0,
        },
    ),
    (
        922,
        AreaTrigger::Circle {
            position: RawPosition::no_names(209, 1190.3, 840.6, 13.4),
            radius: 20.0,
        },
    ),
    (
        924,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -6773.5, -2889.8, 15.1),
            radius: 20.0,
        },
    ),
    (
        942,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4950.6, -1625.1, -3.0),
            radius: 25.0,
        },
    ),
    (
        943,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4950.7, -1595.1, -3.0),
            radius: 25.0,
        },
    ),
    (
        944,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4950.6, -1567.1, -3.0),
            radius: 25.0,
        },
    ),
    (
        962,
        AreaTrigger::Circle {
            position: RawPosition::no_names(209, 1909.3, 1015.1, 11.5),
            radius: 10.0,
        },
    ),
    (
        982,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -2372.5, -1991.6, 121.0),
            radius: 35.0,
        },
    ),
    (
        1022,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 898.5, 922.7, 126.8),
            radius: 35.0,
        },
    ),
    (
        1023,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7162.1, -3845.9, 8.8),
            radius: 20.0,
        },
    ),
    (
        1024,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4370.8, 3289.1, 23.8),
            radius: 35.0,
        },
    ),
    (
        1025,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -4461.9, 242.6, 45.2),
            length: 52.7,
            width: 20.5,
            height: 14.2,
            yaw: 3.6,
        },
    ),
    (
        1042,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 357.2, -2106.1, 121.8),
            radius: 60.0,
        },
    ),
    (
        1064,
        AreaTrigger::Circle {
            position: RawPosition::no_names(249, 29.2, -22.2, 10.5),
            radius: 30.0,
        },
    ),
    (
        1103,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -14468.0, 457.5, 16.0),
            radius: 2.0,
        },
    ),
    (
        1104,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5096.8, 750.2, 260.5),
            radius: 2.0,
        },
    ),
    (
        1105,
        AreaTrigger::Circle {
            position: RawPosition::no_names(90, -377.1, 44.0, -156.5),
            radius: 15.0,
        },
    ),
    (
        1125,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8561.5, 823.4, 106.5),
            radius: 3.0,
        },
    ),
    (
        1205,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -295.6, -3460.7, 193.9),
            radius: 14.0,
        },
    ),
    (
        1306,
        AreaTrigger::Circle {
            position: RawPosition::no_names(109, -496.9, 42.7, -91.1),
            radius: 25.0,
        },
    ),
    (
        1326,
        AreaTrigger::Circle {
            position: RawPosition::no_names(109, -467.5, 95.3, -105.0),
            radius: 7.0,
        },
    ),
    (
        1387,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4260.7, -6273.6, 90.2),
            radius: 20.0,
        },
    ),
    (
        1388,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4277.6, -6296.0, 95.6),
            radius: 20.0,
        },
    ),
    (
        1426,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11204.5, -2730.6, 14.9),
            radius: 2.0,
        },
    ),
    (
        1427,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11079.7, -2855.6, 10.7),
            radius: 2.0,
        },
    ),
    (
        1428,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11208.1, -2960.5, 9.2),
            radius: 2.0,
        },
    ),
    (
        1429,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11337.4, -2848.9, 9.3),
            radius: 2.0,
        },
    ),
    (
        1446,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11185.1, -2834.6, 116.5),
            radius: 2.0,
        },
    ),
    (
        1447,
        AreaTrigger::Circle {
            position: RawPosition::no_names(209, 1811.1, 701.0, 15.3),
            radius: 30.0,
        },
    ),
    (
        1466,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -7176.6, -937.7, 170.2),
            radius: 13.0,
        },
    ),
    (
        1468,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -7518.2, -1239.1, 287.2),
            radius: 10.0,
        },
    ),
    (
        1470,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 73.5, -215.0, 52.4),
            radius: 10.0,
        },
    ),
    (
        1472,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 457.0, 48.4, -65.3),
            radius: 12.0,
        },
    ),
    (
        1506,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -7921.1, -2604.2, 223.3),
            radius: 8.0,
        },
    ),
    (
        1526,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 596.4, -188.5, -50.0),
            radius: 8.0,
        },
    ),
    (
        1590,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 376.1, -191.2, -70.5),
            radius: 10.0,
        },
    ),
    (
        1606,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -6657.4, -2157.1, 264.1),
            radius: 30.0,
        },
    ),
    (
        1626,
        AreaTrigger::Circle {
            position: RawPosition::no_names(269, 3122.2, 2324.6, -129.3),
            radius: 70.0,
        },
    ),
    (
        1628,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, -78.6, -401.4, 38.9),
            radius: 20.0,
        },
    ),
    (
        1629,
        AreaTrigger::Circle {
            position: RawPosition::no_names(269, -2061.1, 6636.0, -144.6),
            radius: 70.0,
        },
    ),
    (
        1631,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -8280.8, -4025.1, -181.1),
            radius: 70.0,
        },
    ),
    (
        1632,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -8799.2, -4115.2, -200.0),
            radius: 70.0,
        },
    ),
    (
        1646,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -907.9, -3534.2, 83.8),
            radius: 30.0,
        },
    ),
    (
        1667,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -3464.7, -4120.9, 17.1),
            radius: 9.0,
        },
    ),
    (
        1686,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 320.0, -215.2, -78.0),
            radius: 10.0,
        },
    ),
    (
        1726,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7286.0, -2125.1, -272.1),
            radius: 12.0,
        },
    ),
    (
        1727,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7208.4, -2027.1, -271.7),
            radius: 12.0,
        },
    ),
    (
        1728,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7170.0, -1852.8, -273.0),
            radius: 12.0,
        },
    ),
    (
        1729,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7395.2, -1735.7, -279.8),
            radius: 12.0,
        },
    ),
    (
        1730,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7449.9, -2111.6, -271.8),
            radius: 12.0,
        },
    ),
    (
        1731,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7463.4, -1957.6, -272.5),
            radius: 12.0,
        },
    ),
    (
        1732,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7509.6, -1946.6, -270.5),
            radius: 12.0,
        },
    ),
    (
        1733,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7516.9, -1829.8, -272.6),
            radius: 12.0,
        },
    ),
    (
        1734,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7594.1, -1771.6, -273.6),
            radius: 12.0,
        },
    ),
    (
        1735,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7748.9, -1721.1, -271.9),
            radius: 12.0,
        },
    ),
    (
        1736,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7768.6, -1957.6, -272.1),
            radius: 12.0,
        },
    ),
    (
        1737,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7612.6, -1930.9, -271.8),
            radius: 12.0,
        },
    ),
    (
        1738,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7952.2, -1792.0, -272.9),
            radius: 12.0,
        },
    ),
    (
        1739,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7895.3, -1623.6, -269.7),
            radius: 12.0,
        },
    ),
    (
        1740,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7874.2, -1806.3, -271.5),
            radius: 12.0,
        },
    ),
    (
        1746,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 411.6, -201.6, -65.1),
            radius: 10.0,
        },
    ),
    (
        1766,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7308.7, -2001.5, -272.5),
            radius: 12.0,
        },
    ),
    (
        1786,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 690.5, -280.1, -43.2),
            radius: 7.0,
        },
    ),
    (
        1826,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 471.1, -9.3, -69.8),
            radius: 10.0,
        },
    ),
    (
        1827,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 1380.2, -554.6, -89.6),
            radius: 20.0,
        },
    ),
    (
        1828,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 497.4, 13.2, -70.1),
            radius: 10.0,
        },
    ),
    (
        1906,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6818.6, -391.7, 40.1),
            radius: 50.0,
        },
    ),
    (
        1926,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 845.9, -317.4, -50.3),
            radius: 10.0,
        },
    ),
    (
        1946,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 70.9, -409.0, 64.3),
            radius: 10.0,
        },
    ),
    (
        1966,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 4989.0, 547.0, 5.4),
            radius: 11.0,
        },
    ),
    (
        1986,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 91.1, -408.4, 64.3),
            radius: 10.0,
        },
    ),
    (
        1987,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 109.6, -415.7, 64.3),
            radius: 10.0,
        },
    ),
    (
        2026,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 153.8, -419.8, 110.5),
            radius: 20.0,
        },
    ),
    (
        2046,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 102.4, -319.2, 65.5),
            radius: 10.0,
        },
    ),
    (
        2066,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 75.3, -547.7, 110.9),
            radius: 30.0,
        },
    ),
    (
        2067,
        AreaTrigger::Circle {
            position: RawPosition::no_names(229, 26.5, -522.5, 110.9),
            radius: 15.0,
        },
    ),
    (
        2068,
        AreaTrigger::Square {
            position: RawPosition::no_names(229, 25.9, -299.3, 24.1),
            length: 20.0,
            width: 20.0,
            height: 20.0,
            yaw: 0.0,
        },
    ),
    (
        2146,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 16299.2, 16277.3, 74.0),
            length: 41.3,
            width: 4.7,
            height: 9.3,
            yaw: 0.0,
        },
    ),
    (
        2166,
        AreaTrigger::Square {
            position: RawPosition::no_names(369, 76.0, 10.5, -4.3),
            length: 9.4,
            width: 19.0,
            height: 20.6,
            yaw: 0.0,
        },
    ),
    (
        2171,
        AreaTrigger::Circle {
            position: RawPosition::no_names(369, 80.2, 2490.9, -1.9),
            radius: 10.0,
        },
    ),
    (
        2173,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8346.5, 514.0, 96.6),
            radius: 10.0,
        },
    ),
    (
        2175,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -4840.3, -1330.5, 508.2),
            radius: 10.0,
        },
    ),
    (
        2187,
        AreaTrigger::Circle {
            position: RawPosition::no_names(329, 3673.6, -3633.9, 139.9),
            radius: 5.0,
        },
    ),
    (
        2206,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5483.9, -749.9, 334.6),
            radius: 30.0,
        },
    ),
    (
        2207,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5552.1, -683.8, 335.2),
            radius: 25.0,
        },
    ),
    (
        2208,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5587.0, -784.0, 335.8),
            radius: 23.0,
        },
    ),
    (
        2209,
        AreaTrigger::Circle {
            position: RawPosition::no_names(329, 3665.1, -3165.0, 127.2),
            radius: 10.0,
        },
    ),
    (
        2210,
        AreaTrigger::Circle {
            position: RawPosition::no_names(329, 3663.9, -3164.3, 127.4),
            radius: 10.0,
        },
    ),
    (
        2211,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6107.6, -4181.6, 852.3),
            radius: 2.0,
        },
    ),
    (
        2213,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5893.3, -4054.8, 596.3),
            radius: 2.0,
        },
    ),
    (
        2214,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 3237.5, -4060.6, 112.0),
            length: 10.0,
            width: 10.0,
            height: 10.0,
            yaw: 0.0,
        },
    ),
    (
        2216,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 3392.5, -3396.8, 143.1),
            length: 22.8,
            width: 8.1,
            height: 34.7,
            yaw: 0.0,
        },
    ),
    (
        2217,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 3392.6, -3361.8, 142.8),
            length: 27.9,
            width: 7.1,
            height: 35.2,
            yaw: 0.0,
        },
    ),
    (
        2221,
        AreaTrigger::Circle {
            position: RawPosition::no_names(329, 3584.8, -3632.1, 142.1),
            radius: 10.0,
        },
    ),
    (
        2226,
        AreaTrigger::Square {
            position: RawPosition::no_names(389, 2.6, -0.0, -13.4),
            length: 30.7,
            width: 12.2,
            height: 25.6,
            yaw: 0.0,
        },
    ),
    (
        2230,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, 1818.4, -4427.3, -10.4),
            length: 21.7,
            width: 11.8,
            height: 21.2,
            yaw: 0.6,
        },
    ),
    (
        2246,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1757.3, -1165.9, 59.6),
            radius: 20.0,
        },
    ),
    (
        2248,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1855.1, -1569.2, 59.2),
            radius: 20.0,
        },
    ),
    (
        2250,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1462.1, -1864.0, 58.7),
            radius: 20.0,
        },
    ),
    (
        2252,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1668.1, -2282.6, 59.2),
            radius: 20.0,
        },
    ),
    (
        2266,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 245.6, 1252.0, 210.1),
            radius: 40.0,
        },
    ),
    (
        2267,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -1596.2, 3145.3, 61.4),
            length: 30.0,
            width: 30.0,
            height: 30.0,
            yaw: 0.0,
        },
    ),
    (
        2286,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -5477.9, -2460.3, 89.3),
            radius: 20.0,
        },
    ),
    (
        2287,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6688.0, -4670.1, 723.9),
            radius: 20.0,
        },
    ),
    (
        2327,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5018.9, -4563.9, 851.8),
            radius: 50.0,
        },
    ),
    (
        2387,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9897.2, -3725.1, 24.7),
            radius: 15.0,
        },
    ),
    (
        2406,
        AreaTrigger::Square {
            position: RawPosition::no_names(33, -287.1, 2175.6, 36.9),
            length: 98.9,
            width: 185.6,
            height: 80.6,
            yaw: 2.9,
        },
    ),
    (
        2407,
        AreaTrigger::Square {
            position: RawPosition::no_names(33, -316.0, 2179.6, 91.8),
            length: 36.1,
            width: 182.8,
            height: 212.6,
            yaw: 2.9,
        },
    ),
    (
        2408,
        AreaTrigger::Square {
            position: RawPosition::no_names(33, -232.4, 2077.1, 68.1),
            length: 44.7,
            width: 123.2,
            height: 43.3,
            yaw: 4.3,
        },
    ),
    (
        2409,
        AreaTrigger::Square {
            position: RawPosition::no_names(33, -255.3, 2035.8, 114.4),
            length: 59.7,
            width: 228.5,
            height: 169.4,
            yaw: 4.3,
        },
    ),
    (
        2410,
        AreaTrigger::Square {
            position: RawPosition::no_names(33, -167.1, 2104.2, 73.4),
            length: 53.0,
            width: 114.7,
            height: 17.1,
            yaw: 6.0,
        },
    ),
    (
        2411,
        AreaTrigger::Square {
            position: RawPosition::no_names(33, -179.7, 2079.4, 42.7),
            length: 19.0,
            width: 162.1,
            height: 187.1,
            yaw: 6.1,
        },
    ),
    (
        2412,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 97.4, -176.3, 127.9),
            length: 10.6,
            width: 16.5,
            height: 22.2,
            yaw: 5.3,
        },
    ),
    (
        2413,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 544.8, -1079.9, 109.5),
            length: 12.2,
            width: 19.1,
            height: 12.1,
            yaw: 3.8,
        },
    ),
    (
        2416,
        AreaTrigger::Square {
            position: RawPosition::no_names(451, 16304.2, 16318.1, 69.4),
            length: 22.8,
            width: 7.6,
            height: 25.6,
            yaw: 2.6,
        },
    ),
    (
        2426,
        AreaTrigger::Square {
            position: RawPosition::no_names(469, -7474.0, -1286.3, 431.3),
            length: 181.8,
            width: 234.7,
            height: 63.9,
            yaw: 5.3,
        },
    ),
    (
        2427,
        AreaTrigger::Square {
            position: RawPosition::no_names(469, -7375.3, -1240.1, 494.3),
            length: 53.9,
            width: 189.9,
            height: 77.1,
            yaw: 0.7,
        },
    ),
    (
        2428,
        AreaTrigger::Square {
            position: RawPosition::no_names(469, -7568.4, -1375.7, 468.5),
            length: 225.3,
            width: 76.0,
            height: 132.2,
            yaw: 5.3,
        },
    ),
    (
        2429,
        AreaTrigger::Square {
            position: RawPosition::no_names(469, -7413.0, -1371.6, 413.4),
            length: 243.3,
            width: 72.9,
            height: 245.9,
            yaw: 3.8,
        },
    ),
    (
        2486,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6207.5, -152.8, 79.8),
            radius: 15.0,
        },
    ),
    (
        2527,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, 1643.4, -4233.6, 56.2),
            length: 12.4,
            width: 11.9,
            height: 15.1,
            yaw: 5.8,
        },
    ),
    (
        2530,
        AreaTrigger::Square {
            position: RawPosition::no_names(450, 215.2, 71.2, 30.1),
            length: 10.0,
            width: 10.0,
            height: 10.0,
            yaw: 3.6,
        },
    ),
    (
        2532,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -8768.1, 409.2, 103.9),
            length: 8.8,
            width: 4.3,
            height: 7.8,
            yaw: 2.3,
        },
    ),
    (
        2534,
        AreaTrigger::Square {
            position: RawPosition::no_names(449, -0.2, -1.6, -0.3),
            length: 4.8,
            width: 4.9,
            height: 8.2,
            yaw: 0.0,
        },
    ),
    (
        2547,
        AreaTrigger::Square {
            position: RawPosition::no_names(289, 332.9, 94.3, 92.2),
            length: 8.2,
            width: 38.4,
            height: 60.4,
            yaw: 0.0,
        },
    ),
    (
        2548,
        AreaTrigger::Square {
            position: RawPosition::no_names(289, 322.9, 112.1, 98.7),
            length: 11.2,
            width: 37.4,
            height: 44.0,
            yaw: 0.8,
        },
    ),
    (
        2549,
        AreaTrigger::Square {
            position: RawPosition::no_names(289, 325.2, 75.6, 93.9),
            length: 9.7,
            width: 31.4,
            height: 54.2,
            yaw: 5.2,
        },
    ),
    (
        2567,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 1282.1, -2548.7, 85.4),
            length: 10.6,
            width: 13.0,
            height: 21.7,
            yaw: 0.5,
        },
    ),
    (
        2568,
        AreaTrigger::Square {
            position: RawPosition::no_names(289, 182.3, 126.4, 143.7),
            length: 12.3,
            width: 8.9,
            height: 16.4,
            yaw: 0.0,
        },
    ),
    (
        2606,
        AreaTrigger::Square {
            position: RawPosition::no_names(30, -723.6, -707.6, 52.9),
            length: 9.6,
            width: 14.1,
            height: 16.2,
            yaw: 5.3,
        },
    ),
    (
        2608,
        AreaTrigger::Square {
            position: RawPosition::no_names(30, 946.8, -500.1, 94.9),
            length: 8.7,
            width: 18.3,
            height: 24.3,
            yaw: 1.1,
        },
    ),
    (
        2610,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 2343.6, -2569.0, 118.8),
            radius: 30.0,
        },
    ),
    (
        2626,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1756.8, -1200.2, 60.7),
            radius: 20.0,
        },
    ),
    (
        2627,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1723.3, -1198.4, 60.0),
            radius: 20.0,
        },
    ),
    (
        2628,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1723.1, -1166.7, 59.6),
            radius: 20.0,
        },
    ),
    (
        2629,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1879.6, -1546.1, 59.0),
            radius: 20.0,
        },
    ),
    (
        2630,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1903.0, -1570.3, 59.7),
            radius: 20.0,
        },
    ),
    (
        2631,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1878.1, -1593.7, 59.5),
            radius: 20.0,
        },
    ),
    (
        2632,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1487.0, -1840.9, 58.7),
            radius: 20.0,
        },
    ),
    (
        2633,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1510.5, -1863.2, 59.1),
            radius: 20.0,
        },
    ),
    (
        2634,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1485.6, -1888.7, 59.1),
            radius: 20.0,
        },
    ),
    (
        2635,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1681.1, -2250.9, 59.0),
            radius: 20.0,
        },
    ),
    (
        2636,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1712.8, -2263.3, 64.1),
            radius: 20.0,
        },
    ),
    (
        2637,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1700.2, -2295.4, 59.0),
            radius: 20.0,
        },
    ),
    (
        2647,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2841.8, -1544.5, 190.5),
            radius: 7.0,
        },
    ),
    (
        2706,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2964.6, -2712.9, 98.7),
            radius: 10.0,
        },
    ),
    (
        2707,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2980.1, -2720.9, 99.9),
            radius: 10.0,
        },
    ),
    (
        2726,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1869.0, -3223.4, 123.1),
            radius: 35.0,
        },
    ),
    (
        2746,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8821.4, 633.3, 94.1),
            radius: 15.0,
        },
    ),
    (
        2786,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9050.9, 445.5, 93.1),
            radius: 1.0,
        },
    ),
    (
        2848,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -4768.6, -3752.1, 53.4),
            radius: 16.0,
        },
    ),
    (
        2886,
        AreaTrigger::Circle {
            position: RawPosition::no_names(230, 1098.0, -466.5, -96.1),
            radius: 20.0,
        },
    ),
    (
        2890,
        AreaTrigger::Circle {
            position: RawPosition::no_names(409, 1115.2, -463.0, -95.0),
            radius: 20.0,
        },
    ),
    (
        2926,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 1999.2, -1121.3, 97.4),
            radius: 9.0,
        },
    ),
    (
        2946,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 2.0, 402.7, 101.0),
            radius: 45.0,
        },
    ),
    (
        3066,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 18.9, -1448.2, 176.1),
            radius: 8.0,
        },
    ),
    (
        3106,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -30.1, -1576.5, 194.5),
            radius: 5.0,
        },
    ),
    (
        3126,
        AreaTrigger::Square {
            position: RawPosition::no_names(349, 756.9, -634.0, -32.8),
            length: 15.9,
            width: 26.4,
            height: 44.1,
            yaw: 1.6,
        },
    ),
    (
        3131,
        AreaTrigger::Square {
            position: RawPosition::no_names(349, 1005.0, -460.5, -43.3),
            length: 10.1,
            width: 31.4,
            height: 23.4,
            yaw: 0.0,
        },
    ),
    (
        3133,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -1484.1, 2617.6, 75.7),
            length: 24.7,
            width: 15.8,
            height: 35.2,
            yaw: 4.5,
        },
    ),
    (
        3134,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -1182.0, 2861.9, 85.3),
            length: 31.7,
            width: 17.3,
            height: 34.2,
            yaw: 3.2,
        },
    ),
    (
        3146,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7180.9, 441.1, 26.5),
            radius: 10.0,
        },
    ),
    (
        3183,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -3730.5, 934.0, 161.0),
            length: 9.4,
            width: 12.8,
            height: 19.7,
            yaw: 0.0,
        },
    ),
    (
        3184,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -3981.6, 771.2, 161.0),
            length: 10.3,
            width: 6.0,
            height: 20.2,
            yaw: 0.0,
        },
    ),
    (
        3185,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -4028.2, 124.0, 26.8),
            length: 9.8,
            width: 4.6,
            height: 15.7,
            yaw: 0.5,
        },
    ),
    (
        3186,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -3837.8, 1250.2, 160.2),
            length: 8.2,
            width: 9.1,
            height: 18.4,
            yaw: 0.0,
        },
    ),
    (
        3187,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -3742.0, 1249.2, 160.2),
            length: 7.9,
            width: 10.3,
            height: 18.7,
            yaw: 0.0,
        },
    ),
    (
        3189,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -3520.6, 1068.7, 161.1),
            length: 9.9,
            width: 11.9,
            height: 23.2,
            yaw: 0.0,
        },
    ),
    (
        3190,
        AreaTrigger::Square {
            position: RawPosition::no_names(429, -56.0, 159.9, -3.5),
            length: 7.2,
            width: 10.1,
            height: 19.6,
            yaw: 0.0,
        },
    ),
    (
        3191,
        AreaTrigger::Square {
            position: RawPosition::no_names(429, 24.6, 159.4, -3.5),
            length: 7.7,
            width: 9.7,
            height: 21.6,
            yaw: 0.0,
        },
    ),
    (
        3193,
        AreaTrigger::Square {
            position: RawPosition::no_names(429, 255.2, -9.1, -2.6),
            length: 8.3,
            width: 8.6,
            height: 20.7,
            yaw: 0.0,
        },
    ),
    (
        3194,
        AreaTrigger::Square {
            position: RawPosition::no_names(429, 37.4, -154.8, -2.7),
            length: 8.9,
            width: 9.2,
            height: 19.4,
            yaw: 0.0,
        },
    ),
    (
        3195,
        AreaTrigger::Square {
            position: RawPosition::no_names(429, -202.7, -314.9, -2.7),
            length: 9.8,
            width: 10.2,
            height: 17.4,
            yaw: 0.0,
        },
    ),
    (
        3196,
        AreaTrigger::Square {
            position: RawPosition::no_names(429, 4.3, -837.1, -33.0),
            length: 6.2,
            width: 11.4,
            height: 22.9,
            yaw: 0.0,
        },
    ),
    (
        3197,
        AreaTrigger::Square {
            position: RawPosition::no_names(429, 194.4, -240.8, -25.6),
            length: 12.9,
            width: 8.3,
            height: 25.3,
            yaw: 1.1,
        },
    ),
    (
        3326,
        AreaTrigger::Circle {
            position: RawPosition::no_names(30, -767.1, -358.7, 68.7),
            radius: 5.0,
        },
    ),
    (
        3327,
        AreaTrigger::Circle {
            position: RawPosition::no_names(30, -1221.3, -352.9, 57.7),
            radius: 5.0,
        },
    ),
    (
        3328,
        AreaTrigger::Circle {
            position: RawPosition::no_names(30, -1305.2, -268.9, 92.0),
            radius: 5.0,
        },
    ),
    (
        3329,
        AreaTrigger::Circle {
            position: RawPosition::no_names(30, 213.9, -373.7, 56.4),
            radius: 5.0,
        },
    ),
    (
        3330,
        AreaTrigger::Circle {
            position: RawPosition::no_names(30, 326.3, -503.8, 71.1),
            radius: 4.0,
        },
    ),
    (
        3331,
        AreaTrigger::Circle {
            position: RawPosition::no_names(30, 683.1, -129.8, 63.6),
            radius: 5.0,
        },
    ),
    (
        3366,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2832.1, -1540.5, 190.2),
            radius: 7.0,
        },
    ),
    (
        3367,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2843.4, -1553.3, 190.7),
            radius: 7.0,
        },
    ),
    (
        3506,
        AreaTrigger::Circle {
            position: RawPosition::no_names(429, 580.8, 487.5, 29.5),
            radius: 8.0,
        },
    ),
    (
        3507,
        AreaTrigger::Circle {
            position: RawPosition::no_names(429, 580.7, 475.9, 29.5),
            radius: 8.0,
        },
    ),
    (
        3508,
        AreaTrigger::Circle {
            position: RawPosition::no_names(429, 592.5, 486.3, 29.5),
            radius: 8.0,
        },
    ),
    (
        3509,
        AreaTrigger::Circle {
            position: RawPosition::no_names(429, 591.9, 476.4, 29.5),
            radius: 8.0,
        },
    ),
    (
        3528,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -7510.9, -1035.9, 183.0),
            length: 3.8,
            width: 0.4,
            height: 2.1,
            yaw: 0.7,
        },
    ),
    (
        3529,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -7512.2, -1034.8, 177.2),
            radius: 4.0,
        },
    ),
    (
        3530,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8536.2, 452.2, 104.9),
            radius: 10.0,
        },
    ),
    (
        3546,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 9933.9, 2501.0, 1317.8),
            radius: 12.0,
        },
    ),
    (
        3547,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1642.1, 239.8, 62.6),
            radius: 13.0,
        },
    ),
    (
        3548,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -4754.6, -3313.8, 313.6),
            radius: 14.0,
        },
    ),
    (
        3549,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 1253.8, -2227.5, 92.2),
            radius: 15.0,
        },
    ),
    (
        3550,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -959.3, -3759.2, 5.0),
            radius: 10.0,
        },
    ),
    (
        3551,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -998.2, -3822.1, 5.3),
            radius: 10.0,
        },
    ),
    (
        3552,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11413.6, 1954.9, 14.2),
            radius: 22.0,
        },
    ),
    (
        3586,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6221.8, -1152.8, 383.2),
            radius: 4.0,
        },
    ),
    (
        3587,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 6221.9, -1153.0, 383.3),
            radius: 5.0,
        },
    ),
    (
        3606,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -7514.8, -1033.1, 163.8),
            radius: 10.0,
        },
    ),
    (
        3626,
        AreaTrigger::Circle {
            position: RawPosition::no_names(469, -7529.1, -1007.2, 408.6),
            radius: 10.0,
        },
    ),
    (
        3646,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1539.9, 1481.4, 352.7),
            radius: 5.0,
        },
    ),
    (
        3647,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 918.5, 1434.0, 346.1),
            radius: 5.0,
        },
    ),
    (
        3649,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1532.1, 1151.7, 374.0),
            radius: 10.0,
        },
    ),
    (
        3650,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, 1422.3, -1855.3, 135.8),
            length: 6.6,
            width: 4.8,
            height: 6.9,
            yaw: 3.0,
        },
    ),
    (
        3654,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, 1034.2, -2087.7, 123.4),
            length: 6.0,
            width: 9.3,
            height: 18.3,
            yaw: 1.6,
        },
    ),
    (
        3669,
        AreaTrigger::Square {
            position: RawPosition::no_names(489, 1009.2, 1290.8, 347.0),
            length: 5.1,
            width: 5.8,
            height: 5.2,
            yaw: 3.6,
        },
    ),
    (
        3671,
        AreaTrigger::Square {
            position: RawPosition::no_names(489, 1456.5, 1628.5, 360.4),
            length: 6.4,
            width: 4.8,
            height: 11.4,
            yaw: 0.9,
        },
    ),
    (
        3686,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1449.8, 1470.6, 342.6),
            radius: 4.0,
        },
    ),
    (
        3687,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1005.3, 1447.3, 335.9),
            radius: 4.0,
        },
    ),
    (
        3688,
        AreaTrigger::Square {
            position: RawPosition::no_names(489, 1125.2, 1541.5, 307.4),
            length: 0.3,
            width: 0.3,
            height: 0.3,
            yaw: 0.0,
        },
    ),
    (
        3690,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -622.1, -4582.1, 22.6),
            length: 27.4,
            width: 27.2,
            height: 21.6,
            yaw: 3.2,
        },
    ),
    (
        3706,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1317.0, 1551.0, 313.2),
            radius: 4.0,
        },
    ),
    (
        3707,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1319.8, 1378.7, 314.8),
            radius: 4.0,
        },
    ),
    (
        3708,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1112.6, 1351.1, 316.6),
            radius: 4.0,
        },
    ),
    (
        3709,
        AreaTrigger::Circle {
            position: RawPosition::no_names(489, 1140.1, 1560.8, 306.8),
            radius: 4.0,
        },
    ),
    (
        3726,
        AreaTrigger::Square {
            position: RawPosition::no_names(229, 178.7, -474.4, 120.6),
            length: 3.8,
            width: 7.9,
            height: 7.9,
            yaw: 0.0,
        },
    ),
    (
        3728,
        AreaTrigger::Square {
            position: RawPosition::no_names(469, -7676.1, -1109.7, 399.1),
            length: 2.5,
            width: 8.7,
            height: 5.1,
            yaw: 3.8,
        },
    ),
    (
        3746,
        AreaTrigger::Circle {
            position: RawPosition::no_names(36, -29.6, -374.5, 59.4),
            radius: 6.0,
        },
    ),
    (
        3766,
        AreaTrigger::Circle {
            position: RawPosition::no_names(43, -108.6, 160.9, -79.8),
            radius: 9.0,
        },
    ),
    (
        3847,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -7663.4, -1218.4, 287.8),
            radius: 3.0,
        },
    ),
    (
        3866,
        AreaTrigger::Circle {
            position: RawPosition::no_names(529, 1185.4, 1184.7, -56.4),
            radius: 4.0,
        },
    ),
    (
        3867,
        AreaTrigger::Circle {
            position: RawPosition::no_names(529, 817.9, 842.4, -56.5),
            radius: 4.0,
        },
    ),
    (
        3868,
        AreaTrigger::Circle {
            position: RawPosition::no_names(529, 809.0, 1185.3, 11.9),
            radius: 4.0,
        },
    ),
    (
        3869,
        AreaTrigger::Circle {
            position: RawPosition::no_names(529, 1147.0, 816.9, -98.4),
            radius: 5.0,
        },
    ),
    (
        3870,
        AreaTrigger::Circle {
            position: RawPosition::no_names(529, 989.9, 1009.2, -42.6),
            radius: 4.0,
        },
    ),
    (
        3886,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -12432.8, 205.2, 2.4),
            radius: 18.0,
        },
    ),
    (
        3926,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11594.1, -1621.4, 47.1),
            radius: 15.0,
        },
    ),
    (
        3928,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -11915.8, -1236.4, 96.2),
            radius: 10.0,
        },
    ),
    (
        3930,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11917.0, -1221.0, 97.6),
            radius: 10.0,
        },
    ),
    (
        3948,
        AreaTrigger::Square {
            position: RawPosition::no_names(529, 1394.4, 1303.6, -8.0),
            length: 7.3,
            width: 16.9,
            height: 17.6,
            yaw: 0.4,
        },
    ),
    (
        3949,
        AreaTrigger::Square {
            position: RawPosition::no_names(529, 627.2, 700.0, -12.0),
            length: 7.9,
            width: 14.2,
            height: 17.3,
            yaw: 3.3,
        },
    ),
    (
        3953,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -1189.3, -2532.2, 22.7),
            length: 7.5,
            width: 13.5,
            height: 17.3,
            yaw: 0.0,
        },
    ),
    (
        3954,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, -811.4, -3505.8, 73.1),
            length: 5.2,
            width: 14.3,
            height: 16.9,
            yaw: 0.7,
        },
    ),
    (
        3956,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11918.6, -1433.9, 44.5),
            radius: 15.0,
        },
    ),
    (
        3957,
        AreaTrigger::Square {
            position: RawPosition::no_names(309, -11916.8, -1256.7, 92.5),
            length: 7.8,
            width: 11.2,
            height: 0.3,
            yaw: 0.0,
        },
    ),
    (
        3958,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11915.0, -1312.7, 77.5),
            radius: 10.0,
        },
    ),
    (
        3959,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11945.7, -1597.6, 36.8),
            radius: 10.0,
        },
    ),
    (
        3960,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11788.9, -1595.1, 37.0),
            radius: 10.0,
        },
    ),
    (
        3961,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11614.2, -1361.5, 76.8),
            radius: 15.0,
        },
    ),
    (
        3962,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11957.6, -1820.0, 54.3),
            radius: 15.0,
        },
    ),
    (
        3963,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -12112.9, -1794.6, 80.4),
            radius: 15.0,
        },
    ),
    (
        3964,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11791.0, -1562.9, 19.7),
            radius: 10.0,
        },
    ),
    (
        3965,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -11610.0, -1749.9, 38.7),
            radius: 20.0,
        },
    ),
    (
        3966,
        AreaTrigger::Circle {
            position: RawPosition::no_names(309, -12072.3, -1479.1, 106.5),
            radius: 10.0,
        },
    ),
    (
        3985,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -6865.0, 731.6, 50.0),
            radius: 20.0,
        },
    ),
    (
        3986,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -8194.3, -4676.4, 11.4),
            radius: 30.0,
        },
    ),
    (
        3991,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1733.3, 526.5, 34.2),
            radius: 35.0,
        },
    ),
    (
        4006,
        AreaTrigger::Square {
            position: RawPosition::no_names(509, -8419.6, 1504.3, 31.7),
            length: 18.0,
            width: 41.8,
            height: 40.5,
            yaw: 5.8,
        },
    ),
    (
        4008,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -8424.9, 1508.7, 32.0),
            length: 12.1,
            width: 50.1,
            height: 35.6,
            yaw: 2.6,
        },
    ),
    (
        4010,
        AreaTrigger::Square {
            position: RawPosition::no_names(1, -8230.4, 2005.3, 129.9),
            length: 16.7,
            width: 31.2,
            height: 46.6,
            yaw: 0.9,
        },
    ),
    (
        4012,
        AreaTrigger::Square {
            position: RawPosition::no_names(531, -8231.3, 2003.6, 129.9),
            length: 38.9,
            width: 10.3,
            height: 81.6,
            yaw: 5.7,
        },
    ),
    (
        4015,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 5085.8, -5110.6, 930.0),
            radius: 6.0,
        },
    ),
    (
        4016,
        AreaTrigger::Circle {
            position: RawPosition::no_names(109, -662.1, 3.8, -90.8),
            radius: 20.0,
        },
    ),
    (
        4017,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -10636.2, -391.3, 101.9),
            radius: 10.0,
        },
    ),
    (
        4020,
        AreaTrigger::Square {
            position: RawPosition::no_names(529, 1313.9, 1310.8, -7.7),
            length: 6.3,
            width: 6.4,
            height: 7.6,
            yaw: 0.0,
        },
    ),
    (
        4021,
        AreaTrigger::Square {
            position: RawPosition::no_names(529, 684.0, 681.2, -12.9),
            length: 5.5,
            width: 5.5,
            height: 4.7,
            yaw: 0.0,
        },
    ),
    (
        4026,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5049.3, -818.5, 495.3),
            radius: 2.0,
        },
    ),
    (
        4027,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 1332.3, -4607.7, 24.4),
            radius: 2.0,
        },
    ),
    (
        4028,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2051.4, 272.6, 57.2),
            radius: 2.0,
        },
    ),
    (
        4029,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -14294.3, 516.1, 9.0),
            radius: 2.0,
        },
    ),
    (
        4030,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -3755.1, -775.7, 9.9),
            radius: 2.0,
        },
    ),
    (
        4031,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7166.1, -3917.9, 9.4),
            radius: 2.0,
        },
    ),
    (
        4032,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -8823.5, 540.3, 96.9),
            radius: 2.0,
        },
    ),
    (
        4033,
        AreaTrigger::Circle {
            position: RawPosition::no_names(531, -8546.2, 1987.7, -96.5),
            radius: 5.0,
        },
    ),
    (
        4034,
        AreaTrigger::Circle {
            position: RawPosition::no_names(531, -8545.7, 1987.7, -32.9),
            radius: 15.0,
        },
    ),
    (
        4036,
        AreaTrigger::Circle {
            position: RawPosition::no_names(531, -8578.0, 1986.8, 100.2),
            radius: 7.0,
        },
    ),
    (
        4047,
        AreaTrigger::Circle {
            position: RawPosition::no_names(531, -8968.0, 1312.6, -104.7),
            radius: 20.0,
        },
    ),
    (
        4052,
        AreaTrigger::Square {
            position: RawPosition::no_names(531, -8176.8, 1673.4, -32.2),
            length: 19.4,
            width: 48.1,
            height: 26.4,
            yaw: 6.1,
        },
    ),
    (
        4055,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 3133.1, -3730.3, 138.7),
            length: 10.0,
            width: 10.0,
            height: 10.0,
            yaw: 0.0,
        },
    ),
    (
        4057,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 3133.3, -3730.8, 138.7),
            length: 10.0,
            width: 10.0,
            height: 10.0,
            yaw: 0.0,
        },
    ),
    (
        4058,
        AreaTrigger::Square {
            position: RawPosition::no_names(0, 2298.2, -5341.0, 90.9),
            length: 33.5,
            width: 17.7,
            height: 24.6,
            yaw: 2.2,
        },
    ),
    (
        4085,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 7763.3, -4010.7, 694.3),
            radius: 95.0,
        },
    ),
    (
        4086,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3292.7, -3151.6, 297.6),
            radius: 10.0,
        },
    ),
    (
        4087,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3291.1, -3151.3, 297.6),
            radius: 100.0,
        },
    ),
    (
        4088,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3321.3, -3119.9, 297.6),
            radius: 90.0,
        },
    ),
    (
        4089,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 864.6, 1321.6, 18.1),
            radius: 5.0,
        },
    ),
    (
        4090,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 2716.6, 1496.8, 236.8),
            radius: 30.0,
        },
    ),
    (
        4092,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9061.9, 349.8, 93.1),
            radius: 6.0,
        },
    ),
    (
        4094,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9119.0, 327.8, 93.2),
            radius: 6.0,
        },
    ),
    (
        4095,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9182.6, 414.9, 89.6),
            radius: 6.0,
        },
    ),
    (
        4096,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -9218.1, 318.6, 73.9),
            radius: 6.0,
        },
    ),
    (
        4097,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3320.9, -3119.1, 297.6),
            radius: 100.0,
        },
    ),
    (
        4098,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5163.2, -645.9, 397.0),
            radius: 6.0,
        },
    ),
    (
        4099,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, -5175.2, -588.1, 398.0),
            radius: 6.0,
        },
    ),
    (
        4100,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1980.0, 305.3, 41.2),
            radius: 6.0,
        },
    ),
    (
        4101,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 1218.0, -4339.3, 25.9),
            radius: 6.0,
        },
    ),
    (
        4102,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 1169.6, -4320.1, 20.9),
            radius: 6.0,
        },
    ),
    (
        4103,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -1545.4, 51.3, 5.4),
            radius: 6.0,
        },
    ),
    (
        4104,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 9914.2, 1864.7, 1321.3),
            radius: 6.0,
        },
    ),
    (
        4105,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, 9948.0, 1932.5, 1328.7),
            radius: 6.0,
        },
    ),
    (
        4112,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3716.4, -5106.8, 141.3),
            radius: 10.0,
        },
    ),
    (
        4113,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3432.8, -3007.2, 295.6),
            radius: 10.0,
        },
    ),
    (
        4114,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3432.3, -3007.3, 295.6),
            radius: 10.0,
        },
    ),
    (
        4115,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3322.7, -3700.4, 262.7),
            radius: 20.0,
        },
    ),
    (
        4116,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 2740.0, -3383.4, 267.7),
            radius: 10.0,
        },
    ),
    (
        4117,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 2917.0, -3344.6, 298.1),
            radius: 12.0,
        },
    ),
    (
        4119,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3214.3, -3476.7, 287.1),
            radius: 10.0,
        },
    ),
    (
        4120,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3493.4, -5375.4, 138.2),
            radius: 50.0,
        },
    ),
    (
        4156,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3006.1, -3434.2, 306.2),
            radius: 6.0,
        },
    ),
    (
        4158,
        AreaTrigger::Square {
            position: RawPosition::no_names(533, 3754.6, -5115.4, 142.4),
            length: 30.7,
            width: 19.2,
            height: 16.9,
            yaw: 6.1,
        },
    ),
    (
        4162,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7142.0, 1397.9, 4.3),
            radius: 4.0,
        },
    ),
    (
        4163,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3320.7, -3115.1, 297.6),
            radius: 100.0,
        },
    ),
    (
        4166,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3310.2, -2990.2, 294.7),
            radius: 100.0,
        },
    ),
    (
        4167,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 3498.2, -5349.0, 145.0),
            radius: 50.0,
        },
    ),
    (
        4168,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7588.5, 756.8, -16.4),
            radius: 4.0,
        },
    ),
    (
        4177,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 2661.0, -3467.2, 262.1),
            radius: 90.0,
        },
    ),
    (
        4180,
        AreaTrigger::Square {
            position: RawPosition::no_names(533, 3374.0, -3059.3, 294.7),
            length: 26.6,
            width: 41.9,
            height: 41.9,
            yaw: 0.0,
        },
    ),
    (
        4184,
        AreaTrigger::Circle {
            position: RawPosition::no_names(533, 2797.8, -3670.6, 273.7),
            radius: 6.0,
        },
    ),
    (
        4189,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 1782.2, 151.6, -62.3),
            radius: 1.0,
        },
    ),
    (
        4261,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 1693.8, 1053.3, 18.7),
            radius: 5.0,
        },
    ),
    (
        4262,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 1617.2, -323.4, 18.7),
            radius: 5.0,
        },
    ),
    (
        4263,
        AreaTrigger::Circle {
            position: RawPosition::no_names(189, 269.1, -211.9, 19.2),
            radius: 5.0,
        },
    ),
    (
        4294,
        AreaTrigger::Circle {
            position: RawPosition::no_names(0, 2541.9, -4785.3, 117.5),
            radius: 80.0,
        },
    ),
    (
        4299,
        AreaTrigger::Circle {
            position: RawPosition::no_names(1, -7376.8, 447.3, 4.5),
            radius: 60.0,
        },
    ),
];
