;; -----------------------------------------------------------------------

;   Definitions of ATDF data for use with BITSTRACT
;
;                 C. Markwardt  02 Feb 2002
;
;         See TRK-2-25 definition document

;; -----------------------------------------------------------------------

;; Predefine the FORMAT structure so we can use the IDL named
;; structure shorthand.

hformat = { TRK_BIT_FIELD, $
            ITEM: 0L, $      ;; Item number
            START: 0L, $     ;; Bit position, starting at 0
            STOP: 0L, $      ;; Input length in bits
            SIGNLENGTH: 0L,$ ;; Length of sign field in bits
            OUTLENGTH: 0L, $ ;; Convert to output length in bits (MSB is lost)
            TYPE: '', $      ;; Data type to convert to
            NAME: '' $       ;; Name of field
          }

;; Sign length of -1 indicates two's complement number
;;
;; Outlength is essentially ignored, but it is checked against the
;; length of the output data type to warn against a too-small type.
  
;; Note: bit fields are addressed starting from 1 here, for comparison
;; with TRK-2-25, but 1 is subtracted at the end.

;; -----------------------------------------------------------------------
;; Format of the first record on file - FILE RECORD
idform = [ $
           {TRK_BIT_FIELD,  1,    1,   36, 29, 32, 'LONG', 'DATA_LENGTH'}, $
           {TRK_BIT_FIELD,  2,   37,   72, 29, 32, 'LONG', 'RECORD_TYPE'}, $
           {TRK_BIT_FIELD,  3,   73,   84,  0,  8, 'BYTE', 'YEAR_NUM'}, $
           {TRK_BIT_FIELD,  4,   85,  100,  0, 16,  'FIX', 'DAY_NUM'}, $
           {TRK_BIT_FIELD,  5,  101,  108,  0,  8, 'BYTE', 'HOUR'}, $
           {TRK_BIT_FIELD,  6,  109,  120,  0,  8, 'BYTE', 'MINUTE'}, $
           {TRK_BIT_FIELD,  7,  121,  128,  0,  8, 'BYTE', 'SECOND'}, $
           {TRK_BIT_FIELD,  9,  149,  156,  0,  8, 'BYTE', 'SPACECRAFT'}, $
           {TRK_BIT_FIELD, 10,  157,  164,  0,  8, 'BYTE', 'ID1'}, $
           {TRK_BIT_FIELD, 11,  165,  172,  0,  8, 'BYTE', 'ID2'}, $
           {TRK_BIT_FIELD, 12,  173,  180,  0,  8, 'BYTE', 'ID3'}, $
           {TRK_BIT_FIELD, 13,  181,  192,  0,  8, 'BYTE', 'ID4'}, $
           {TRK_BIT_FIELD, 14,  193,  208,  0,  8, 'BYTE', 'ID5'}, $
           {TRK_BIT_FIELD, 15,  209,  216,  0,  8, 'BYTE', 'ID6'}, $
           {TRK_BIT_FIELD, 16,  217,  228,  0,  8, 'BYTE', 'ID7'}, $
           {TRK_BIT_FIELD, 17,  229,  236,  0,  8, 'BYTE', 'ID8'}  $
         ]
idform.start = idform.start - 1  & idform.stop = idform.stop - 1

;; -----------------------------------------------------------------------
;; Format of the second record on file - TRANSPONDER RECORD
xpform = [ $
           {TRK_BIT_FIELD,  1,    1,   36, 29, 32, 'LONG', 'DATA_LENGTH'}, $
           {TRK_BIT_FIELD,  2,   37,   72, 29, 32, 'LONG', 'RECORD_TYPE'}, $
           {TRK_BIT_FIELD,  3,   73,   84,  0,  8, 'BYTE', 'YEAR_ON'}, $
           {TRK_BIT_FIELD,  4,   85,  100,  0, 16,  'FIX', 'DAY_ON'}, $
           {TRK_BIT_FIELD,  5,  101,  108,  0,  8, 'BYTE', 'HOUR_ON'}, $
           {TRK_BIT_FIELD,  6,  109,  120,  0,  8, 'BYTE', 'MINUTE_ON'}, $
           {TRK_BIT_FIELD,  7,  121,  128,  0,  8, 'BYTE', 'SECOND_ON'}, $
           {TRK_BIT_FIELD,  9,  149,  156,  0,  8, 'BYTE', 'SPACECRAFT'}, $
           {TRK_BIT_FIELD, 11,  181,  192,  0,  8, 'BYTE', 'YEAR_OFF'}, $
           {TRK_BIT_FIELD, 12,  193,  208,  0, 16,  'FIX', 'DAY_OFF'}, $
           {TRK_BIT_FIELD, 13,  209,  216,  0,  8, 'BYTE', 'HOUR_OFF'}, $
           {TRK_BIT_FIELD, 14,  217,  228,  0,  8, 'BYTE', 'MINUTE_OFF'}, $
           {TRK_BIT_FIELD, 15,  229,  236,  0,  8, 'BYTE', 'SECOND_OFF'}, $
           {TRK_BIT_FIELD, 17,  253,  288,  0, 32, 'LONG', 'SC_XPON_HP'}, $
           {TRK_BIT_FIELD, 18,  289,  324,  0, 32, 'LONG', 'SC_XPON_LP'}  $
         ]
xpform.start = xpform.start - 1  & xpform.stop = xpform.stop - 1

;; -----------------------------------------------------------------------
;; Format of remaining records - TRACKING RECORDS
tkform = [ $
           {TRK_BIT_FIELD,  1,    1,   36, 29, 32, 'LONG', 'DATA_LENGTH'}, $
           {TRK_BIT_FIELD,  2,   37,   72, 29, 32, 'LONG', 'RECORD_TYPE'}, $
           {TRK_BIT_FIELD,  3,   73,   84,  0,  8, 'BYTE', 'YEAR_TAG'}, $
           {TRK_BIT_FIELD,  4,   85,  100,  0, 16,  'FIX', 'DAY_TAG'}, $
           {TRK_BIT_FIELD,  5,  101,  108,  0,  8, 'BYTE', 'HOUR_TAG'}, $
           {TRK_BIT_FIELD,  6,  109,  120,  0,  8, 'BYTE', 'MINUTE_TAG'}, $
           {TRK_BIT_FIELD,  7,  121,  128,  0,  8, 'BYTE', 'SECOND_TAG'}, $
           {TRK_BIT_FIELD,  8,  129,  156,  0,  8, 'BYTE', 'SPACECRAFT'}, $
           {TRK_BIT_FIELD,  9,  157,  164,  0,  8, 'BYTE', 'NET_ID'}, $
           {TRK_BIT_FIELD, 10,  165,  172,  0,  8, 'BYTE', 'STATION'}, $
           {TRK_BIT_FIELD, 11,  173,  180,  0,  8, 'BYTE', 'DOWNLINK_BAND'}, $
           {TRK_BIT_FIELD, 12,  181,  184,  0,  8, 'BYTE', 'DATA_TYPE'}, $
           {TRK_BIT_FIELD, 13,  185,  192,  0,  8, 'BYTE', 'GROUND_MODE'}, $
           {TRK_BIT_FIELD, 14,  193,  200,  0,  8, 'BYTE', 'RANGE_TYPE'}, $
           {TRK_BIT_FIELD, 15,  201,  208,  0,  8, 'BYTE', 'ANGLE_TYPE'}, $
           {TRK_BIT_FIELD, 16,  209,  216,  0,  8, 'BYTE', 'DRVID_TYPE'}, $
           {TRK_BIT_FIELD, 17,  217,  221,  0,  8, 'BYTE', 'DOPPLER_GOOD0'}, $
           {TRK_BIT_FIELD, 18,  222,  222,  0,  8, 'BYTE', 'DOPPLER_TOL0'}, $
           {TRK_BIT_FIELD, 19,  223,  223,  0,  8, 'BYTE', 'ZERO_1'}, $
           {TRK_BIT_FIELD, 20,  224,  227, -1, 16,  'FIX', 'DOPPLER_BIAS'}, $
           {TRK_BIT_FIELD, 21,  228,  228,  0,  8, 'BYTE', 'RESERVED_1'}, $
           {TRK_BIT_FIELD, 22,  229,  229,  0,  8, 'BYTE', 'ANGLE_GOOD0'}, $
           {TRK_BIT_FIELD, 23,  230,  232,  0,  8, 'BYTE', 'RESERVED_2'}, $
           {TRK_BIT_FIELD, 24,  233,  235,  0,  8, 'BYTE', 'RESERVED_3'}, $
           {TRK_BIT_FIELD, 25,  236,  236,  0,  8, 'BYTE', 'RCVR_LOCK0'}, $
           {TRK_BIT_FIELD, 26,  237,  237,  0,  8, 'BYTE', 'XMTR_ON0'}, $
           {TRK_BIT_FIELD, 27,  238,  239,  0,  8, 'BYTE', 'RESERVED_4'}, $
           {TRK_BIT_FIELD, 28,  240,  242,  0,  8, 'BYTE', 'SOURCE_DESIG'}, $
           {TRK_BIT_FIELD, 29,  241,  252,  0, 16,  'FIX', 'RESERVED_5'}, $
           {TRK_BIT_FIELD, 30,  253,  288,  0, 32, 'LONG', 'SAMPLER_TIME'}, $
           {TRK_BIT_FIELD, 31,  289,  324,  0, 32, 'LONG', 'DOPPLER_CNT_HP'}, $
           {TRK_BIT_FIELD, 32,  325,  360,  0, 32, 'LONG', 'DOPPLER_CNT_LP'} $
         ]
tkform = [ tkform, $
           {TRK_BIT_FIELD, 33,  361,  396,  0, 32, 'LONG', 'RANGE_DATA1'}, $
           {TRK_BIT_FIELD, 34,  397,  432,  0, 32, 'LONG', 'RANGE_DATA2'}, $
           {TRK_BIT_FIELD, 35,  433,  452,  0, 32, 'LONG', 'LOW_RANGE'}, $
           {TRK_BIT_FIELD, 36,  453,  484,  0, 32, 'LONG', 'RESERVED_6'}, $
           {TRK_BIT_FIELD, 37,  525,  540, -1, 16,  'FIX', 'DRVID_PNR'}, $
           {TRK_BIT_FIELD, 38,  541,  576,  0, 32, 'LONG', 'ANGLE1'}, $
           {TRK_BIT_FIELD, 39,  577,  612,  0, 32, 'LONG', 'ANGLE2'}, $
           {TRK_BIT_FIELD, 40,  613,  648,  0, 32, 'LONG', 'DOPPLER_REF'}, $
           {TRK_BIT_FIELD, 41,  649,  684, -1, 32, 'LONG', 'DRVID'}, $
           {TRK_BIT_FIELD, 42,  685,  720,  0, 32, 'LONG', 'DOPPLER_CNT2_HP'},$
           {TRK_BIT_FIELD, 43,  721,  756,  0, 32, 'LONG', 'DOPPLER_CNT2_LP'},$
           {TRK_BIT_FIELD, 44,  757,  792,  0, 32, 'LONG', 'DOPPLER_CNT3_HP'},$
           {TRK_BIT_FIELD, 45,  793,  828,  0, 32, 'LONG', 'DOPPLER_CNT3_LP'},$
           {TRK_BIT_FIELD, 46,  829,  864,  0, 32, 'LONG', 'DOPPLER_CNT4_HP'},$
           {TRK_BIT_FIELD, 47,  865,  900,  0, 32, 'LONG', 'DOPPLER_CNT4_LP'},$
           {TRK_BIT_FIELD, 48,  901,  936,  0, 32, 'LONG', 'DOPPLER_CNT5_HP'},$
           {TRK_BIT_FIELD, 49,  937,  972,  0, 32, 'LONG', 'DOPPLER_CNT5_LP'},$
           {TRK_BIT_FIELD, 50,  973, 1008,  0, 32, 'LONG', 'DOPPLER_CNT6_HP'},$
           {TRK_BIT_FIELD, 51, 1009, 1044,  0, 32, 'LONG', 'DOPPLER_CNT6_LP'},$
           {TRK_BIT_FIELD, 52, 1045, 1080,  0, 32, 'LONG', 'DOPPLER_CNT7_HP'},$
           {TRK_BIT_FIELD, 53, 1081, 1116,  0, 32, 'LONG', 'DOPPLER_CNT7_LP'},$
           {TRK_BIT_FIELD, 54, 1117, 1152,  0, 32, 'LONG', 'DOPPLER_CNT8_HP'},$
           {TRK_BIT_FIELD, 55, 1153, 1188,  0, 32, 'LONG', 'DOPPLER_CNT8_LP'},$
           {TRK_BIT_FIELD, 56, 1189, 1224,  0, 32, 'LONG', 'DOPPLER_CNT9_HP'},$
           {TRK_BIT_FIELD, 57, 1225, 1260,  0, 32, 'LONG', 'DOPPLER_CNT9_LP'},$
           {TRK_BIT_FIELD, 58, 1261, 1296,  0, 32, 'LONG', 'DOPPLER_CNTA_HP'},$
           {TRK_BIT_FIELD, 59, 1297, 1332,  0, 32, 'LONG', 'DOPPLER_CNTA_LP'}$
         ]
tkform = [ tkform, $
           {TRK_BIT_FIELD, 60, 1333, 1368, -1, 32, 'LONG', 'DOPPLER_RESID'}, $
           {TRK_BIT_FIELD, 61, 1369, 1404, -1, 32, 'LONG', 'RANGE_RESID'}, $
           {TRK_BIT_FIELD, 62, 1405, 1422, -1, 32, 'LONG', 'ANGLE1_RESID'}, $
           {TRK_BIT_FIELD, 63, 1423, 1440, -1, 32, 'LONG', 'ANGLE2_RESID'}, $
           {TRK_BIT_FIELD, 64, 1441, 1443,  0,  8, 'BYTE', 'UPLINK_BAND'}, $
           {TRK_BIT_FIELD, 65, 1444, 1446,  0,  8, 'BYTE', 'ANGLE_MODE'}, $
           {TRK_BIT_FIELD, 66, 1447, 1448,  0,  8, 'BYTE', 'CONSCAN_MODE'}, $
           {TRK_BIT_FIELD, 67, 1449, 1449,  0,  8, 'BYTE', 'ANGLE1_RESID_TOL0'}, $
           {TRK_BIT_FIELD, 68, 1450, 1450,  0,  8, 'BYTE', 'ANGLE2_RESID_TOL0'}, $
           {TRK_BIT_FIELD, 69, 1451, 1453,  0,  8, 'BYTE', 'DOPPLER_CHANNEL'},$
           {TRK_BIT_FIELD, 70, 1454, 1454,  0,  8, 'BYTE', 'FREQ_STD'}, $
           {TRK_BIT_FIELD, 71, 1455, 1456,  0,  8, 'BYTE', 'DOPPLER_RCVR_REF'},$
           {TRK_BIT_FIELD, 72, 1457, 1462,  0,  8, 'BYTE', 'RESERVED_7'}, $
           {TRK_BIT_FIELD, 73, 1463, 1463,  0,  8, 'BYTE', 'DOPPLER_RESID_TOL0'},$
           {TRK_BIT_FIELD, 74, 1464, 1464,  0,  8, 'BYTE', 'DOPPLER_NOISE_TOL0'},$
           {TRK_BIT_FIELD, 75, 1465, 1494,  0, 32, 'LONG', 'RESERVED_8'}, $
           {TRK_BIT_FIELD, 76, 1495, 1512,  0, 32, 'LONG', 'SLIPPED_CYCLE'}, $
           {TRK_BIT_FIELD, 77, 1513, 1530,  0, 32, 'LONG', 'DOPPLER_NOISE'}, $
           {TRK_BIT_FIELD, 78, 1531, 1548, -1, 32, 'LONG', 'SIGNAL_STRENGTH'}, $
           {TRK_BIT_FIELD, 79, 1549, 1584, -1, 32, 'LONG', 'DIFF_DOPPLER_PHASE'}, $
           {TRK_BIT_FIELD, 80, 1585, 1585,  0,  8, 'BYTE', 'RANGE_MOD_ON0'}, $
           {TRK_BIT_FIELD, 81, 1586, 1586,  0,  8, 'BYTE', 'PRIME_RANGE_CHAN'}, $
           {TRK_BIT_FIELD, 82, 1587, 1587,  0,  8, 'BYTE', 'PIPELINING_ON0'}, $
           {TRK_BIT_FIELD, 83, 1588, 1588,  0,  8, 'BYTE', 'CHOPPER_FREQ_ON0'}, $
           {TRK_BIT_FIELD, 84, 1589, 1589,  0,  8, 'BYTE', 'RESERVED_9'}, $
           {TRK_BIT_FIELD, 85, 1590, 1590,  0,  8, 'BYTE', 'RANGE_VALID0'}, $
           {TRK_BIT_FIELD, 86, 1591, 1591,  0,  8, 'BYTE', 'RANGE_CALIB_TOL0'}, $
           {TRK_BIT_FIELD, 87, 1592, 1592,  0,  8, 'BYTE', 'RANGE_CONFIG_SAME0'}, $
           {TRK_BIT_FIELD, 88, 1593, 1593,  0,  8, 'BYTE', 'RANGE_PNR_TOL0'}, $
           {TRK_BIT_FIELD, 89, 1594, 1594,  0,  8, 'BYTE', 'RANGE_RESID_TOL0'}, $
           {TRK_BIT_FIELD, 90, 1595, 1595,  0,  8, 'BYTE', 'PSEUDO_DRVID_TOL0'}, $
           {TRK_BIT_FIELD, 91, 1596, 1596,  0,  8, 'BYTE', 'DIFF_RANGE_TOL0'} $
             ]

tkform = [tkform, $
           {TRK_BIT_FIELD, 92, 1597, 1600,  0,  8, 'BYTE', 'RCVR_NUMBER'}, $
           {TRK_BIT_FIELD, 93, 1601, 1601,  0,  8, 'BYTE', 'RESERVED_10'}, $
           {TRK_BIT_FIELD, 94, 1602, 1603,  0,  8, 'BYTE', 'AMP_NUMBER'}, $
           {TRK_BIT_FIELD, 95, 1604, 1605,  0,  8, 'BYTE', 'AMP_TYPE'}, $
           {TRK_BIT_FIELD, 96, 1606, 1606,  0,  8, 'BYTE', 'XMTR_POWER_IND'}, $
           {TRK_BIT_FIELD, 97, 1607, 1607,  0,  8, 'BYTE', 'RESERVED_11'}, $
           {TRK_BIT_FIELD, 98, 1608, 1620,  0, 16,  'FIX', 'XMTR_POWER'}, $
           {TRK_BIT_FIELD, 99, 1621, 1644,  0, 32, 'LONG', 'RANGE_CALIB'}, $
           {TRK_BIT_FIELD,100, 1645, 1656, -1, 16,  'FIX', 'RANGE_PNR'}, $
           {TRK_BIT_FIELD,101, 1657, 1692, -1, 32, 'LONG', 'AVG_DOPPLER_RESID'}, $
           {TRK_BIT_FIELD,102, 1693, 1728, -1, 32, 'LONG', 'PSEUDO_DRVID'}, $
           {TRK_BIT_FIELD,103, 1729, 1764, -1, 32, 'LONG', 'DIFF_S_X_RANGE'}, $
           {TRK_BIT_FIELD,104, 1765, 1786, -1, 32, 'LONG', 'Z_CORRECTION'}, $
           {TRK_BIT_FIELD,105, 1787, 1786,  0,  8, 'BYTE', 'SPACECRAFT_DELAY'}, $
           {TRK_BIT_FIELD,106, 1801, 1833,  0,  8, 'BYTE', 'DRVID_NOISE'}, $
           {TRK_BIT_FIELD,107, 1834, 1834,  0,  8, 'BYTE', 'DRVID_VALID0'}, $
           {TRK_BIT_FIELD,108, 1835, 1835,  0,  8, 'BYTE', 'DRVID_NOISE_TOL0'}, $
           {TRK_BIT_FIELD,109, 1836, 1836,  0,  8, 'BYTE', 'DRIVD_PNR_TOL0'}, $
           {TRK_BIT_FIELD,110, 1837, 1872, -1, 32, 'LONG', 'DIFF_S_X_DRVID'} $
         ]

tkform = [ tkform, $
           {TRK_BIT_FIELD,111, 1873, 1877,  0,  8, 'BYTE', 'RAMP_CTRL'}, $
           {TRK_BIT_FIELD,112, 1878, 1908, -1, 32, 'LONG', 'RAMP_RATE'}, $
           {TRK_BIT_FIELD,113, 1909, 1944,  0, 32, 'LONG', 'RAMP_START1'}, $
           {TRK_BIT_FIELD,114, 1945, 1980,  0, 32, 'LONG', 'RAMP_START2'}, $
           {TRK_BIT_FIELD,115, 1981, 2012,  0, 32, 'LONG', 'RESERVED_12'}, $
           {TRK_BIT_FIELD,116, 2125, 2160,  0, 32, 'LONG', 'XMTR_FREQ'} $
;          {TRK_BIT_FIELD,117, 2161, 2192,  0, 32, 'LONG', 'ZERO'} 
         ]

tkform.start = tkform.start - 1  & tkform.stop = tkform.stop - 1
