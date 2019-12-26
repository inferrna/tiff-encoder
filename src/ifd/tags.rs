//! Constants for commonly used tags in TIFF files, baseline
//! or extended.
//!
//! Check the [Tiff Tag Reference](https://www.awaresystems.be/imaging/tiff/tifftags.html)
//! for more information on each tag.
#![allow(non_upper_case_globals)]

/// 16-bit identifier of a field entry.
pub type FieldTag = u16;

// pub const NewSubfileType: u16 = 0x00FE;
// pub const ImageWidth: u16 = 0x0100;
// pub const ImageLength: u16 = 0x0101;
// pub const BitsPerSample: u16 = 0x0102;
// pub const Compression: u16 = 0x0103;
// pub const PhotometricInterpretation: u16 = 0x0106;
// pub const FillOrder: u16 = 0x010A;
// pub const ImageDescription: u16 = 0x010E;
// pub const Make: u16 = 0x010F;
// pub const Model: u16 = 0x0110;
// pub const StripOffsets: u16 = 0x0111;
// pub const Orientation: u16 = 0x0112;
// pub const SamplesPerPixel: u16 = 0x0115;
// pub const RowsPerStrip: u16 = 0x0116;
// pub const StripByteCounts: u16 = 0x0117;
// pub const XResolution: u16 = 0x011A;
// pub const YResolution: u16 = 0x011B;
// pub const PlanarConfiguration: u16 = 0x011C;
// pub const ResolutionUnit: u16 = 0x0128;
// pub const Software: u16 = 0x0131;
// pub const DateTime: u16 = 0x0132;
// pub const Artist: u16 = 0x013B;
// pub const TileWidth: u16 = 0x0142;
// pub const TileLength: u16 = 0x0143;
// pub const TileOffsets: u16 = 0x0144;
// pub const TileByteCounts: u16 = 0x0145;
// pub const Copyright: u16 = 0x8298;

pub const SubfileType: u16 = 0x00FF;
pub const Threshholding: u16 = 0x0107;
pub const CellWidth: u16 = 0x0108;
pub const CellLength: u16 = 0x0109;
pub const DocumentName: u16 = 0x010D;
pub const MinSampleValue: u16 = 0x0118;
pub const MaxSampleValue: u16 = 0x0119;
pub const PageName: u16 = 0x011D;
pub const XPosition: u16 = 0x011E;
pub const YPosition: u16 = 0x011F;
pub const FreeOffsets: u16 = 0x0120;
pub const FreeByteCounts: u16 = 0x0121;
pub const GrayResponseUnit: u16 = 0x0122;
pub const GrayResponseCurve: u16 = 0x0123;
pub const T4Options: u16 = 0x0124;
pub const T6Options: u16 = 0x0125;
pub const PageNumber: u16 = 0x0129;
pub const TransferFunction: u16 = 0x012D;
pub const HostComputer: u16 = 0x013C;
pub const Predictor: u16 = 0x013D;
pub const WhitePoint: u16 = 0x013E;
pub const PrimaryChromaticities: u16 = 0x013F;
pub const ColorMap: u16 = 0x0140;
pub const HalftoneHints: u16 = 0x0141;
pub const BadFaxLines: u16 = 0x0146;
pub const CleanFaxData: u16 = 0x0147;
pub const ConsecutiveBadFaxLines: u16 = 0x0148;
pub const SubIFDs: u16 = 0x014A;
pub const InkSet: u16 = 0x014C;
pub const InkNames: u16 = 0x014D;
pub const NumberOfInks: u16 = 0x014E;
pub const DotRange: u16 = 0x0150;
pub const TargetPrinter: u16 = 0x0151;
pub const ExtraSamples: u16 = 0x0152;
pub const SampleFormat: u16 = 0x0153;
pub const SMinSampleValue: u16 = 0x0154;
pub const SMaxSampleValue: u16 = 0x0155;
pub const TransferRange: u16 = 0x0156;
pub const ClipPath: u16 = 0x0157;
pub const XClipPathUnits: u16 = 0x0158;
pub const YClipPathUnits: u16 = 0x0159;
pub const Indexed: u16 = 0x015A;
pub const JPEGTables: u16 = 0x015B;
pub const OPIProxy: u16 = 0x015F;
pub const GlobalParametersIFD: u16 = 0x0190;
pub const ProfileType: u16 = 0x0191;
pub const FaxProfile: u16 = 0x0192;
pub const CodingMethods: u16 = 0x0193;
pub const VersionYear: u16 = 0x0194;
pub const ModeNumber: u16 = 0x0195;
pub const Decode: u16 = 0x01B1;
pub const DefaultImageColor: u16 = 0x01B2;
pub const JPEGProc: u16 = 0x0200;
pub const JPEGInterchangeFormat: u16 = 0x0201;
pub const JPEGInterchangeFormatLength: u16 = 0x0202;
pub const JPEGRestartInterval: u16 = 0x0203;
pub const JPEGLosslessPredictors: u16 = 0x0205;
pub const JPEGPointTransforms: u16 = 0x0206;
pub const JPEGQTables: u16 = 0x0207;
pub const JPEGDCTables: u16 = 0x0208;
pub const JPEGACTables: u16 = 0x0209;
pub const YCbCrCoefficients: u16 = 0x0211;
pub const YCbCrSubSampling: u16 = 0x0212;
pub const YCbCrPositioning: u16 = 0x0213;
pub const ReferenceBlackWhite: u16 = 0x0214;
pub const StripRowCounts: u16 = 0x022F;
pub const XMP: u16 = 0x02BC;
pub const ImageID: u16 = 0x800D;
pub const ImageLayer: u16 = 0x87AC;

// extracted from https://github.com/schoolpost/PyDNG/raw/master/pydng.py
// cat /tmp/tiffs.txt | tr "(),=" \ | awk '{print "printf \"pub const "$1": u16 = 0x%04x; //"$3"\\n\" "$2}'| bash

pub const NewSubfileType: u16 = 0x00fe; //Type.Long
pub const ImageWidth: u16 = 0x0100; //Type.Long
pub const ImageLength: u16 = 0x0101; //Type.Long
pub const BitsPerSample: u16 = 0x0102; //Type.Short
pub const Compression: u16 = 0x0103; //Type.Short
pub const PhotometricInterpretation: u16 = 0x0106; //Type.Short
pub const FillOrder: u16 = 0x010a; //Type.Short
pub const ImageDescription: u16 = 0x010e; //Type.Ascii
pub const Make: u16 = 0x010f; //Type.Ascii
pub const Model: u16 = 0x0110; //Type.Ascii
pub const StripOffsets: u16 = 0x0111; //Type.Long
pub const Orientation: u16 = 0x0112; //Type.Short
pub const SamplesPerPixel: u16 = 0x0115; //Type.Short
pub const RowsPerStrip: u16 = 0x0116; //Type.Short
pub const StripByteCounts: u16 = 0x0117; //Type.Long
pub const XResolution: u16 = 0x011a; //Type.Rational
pub const YResolution: u16 = 0x011b; //Type.Rational
pub const PlanarConfiguration: u16 = 0x011c; //Type.Short
pub const ResolutionUnit: u16 = 0x0128; //Type.Short
pub const Software: u16 = 0x0131; //Type.Ascii
pub const DateTime: u16 = 0x0132; //Type.Ascii
pub const Artist: u16 = 0x013b; //Type.Ascii
pub const TileWidth: u16 = 0x0142; //Type.Short
pub const TileLength: u16 = 0x0143; //Type.Short
pub const TileOffsets: u16 = 0x0144; //Type.Long
pub const TileByteCounts: u16 = 0x0145; //Type.Long
pub const Copyright: u16 = 0x8298; //Type.Ascii
pub const SubIFD: u16 = 0x014a; //Type.IFD
pub const XMP_Metadata: u16 = 0x02bc; //Type.Undefined
pub const CFARepeatPatternDim: u16 = 0x828d; //Type.Short
pub const CFAPattern: u16 = 0x828e; //Type.Byte
pub const ExposureTime: u16 = 0x829a; //Type.Rational
pub const FNumber: u16 = 0x829d; //Type.Rational
pub const EXIF_IFD: u16 = 0x8769; //Type.IFD
pub const ExposureProgram: u16 = 0x8822; //Type.Short
pub const PhotographicSensitivity: u16 = 0x8827; //Type.Short
pub const SensitivityType: u16 = 0x8830; //Type.Short
pub const ExifVersion: u16 = 0x9000; //Type.Undefined
pub const DateTimeOriginal: u16 = 0x9003; //Type.Ascii
pub const ShutterSpeedValue: u16 = 0x9201; //Type.Srational
pub const ApertureValue: u16 = 0x9202; //Type.Rational
pub const ExposureBiasValue: u16 = 0x9204; //Type.Srational
pub const MaxApertureValue: u16 = 0x9205; //Type.Rational
pub const SubjectDistance: u16 = 0x9206; //Type.Rational
pub const MeteringMode: u16 = 0x9207; //Type.Short
pub const Flash: u16 = 0x9209; //Type.Short
pub const FocalLength: u16 = 0x920a; //Type.Rational
pub const TIFF_EP_StandardID: u16 = 0x9216; //Type.Byte
pub const SubsecTime: u16 = 0x9290; //Type.Ascii
pub const SubsecTimeOriginal: u16 = 0x9291; //Type.Ascii
pub const FocalPlaneXResolution: u16 = 0xa20e; //Type.Rational
pub const FocalPlaneYResolution: u16 = 0xa20f; //Type.Rational
pub const FocalPlaneResolutionUnit: u16 = 0xa210; //Type.Short
pub const FocalLengthIn35mmFilm: u16 = 0xa405; //Type.Short
pub const EXIFPhotoBodySerialNumber: u16 = 0xa431; //Type.Ascii
pub const EXIFPhotoLensModel: u16 = 0xa434; //Type.Ascii
pub const DNGVersion: u16 = 0xc612; //Type.Byte
pub const DNGBackwardVersion: u16 = 0xc613; //Type.Byte
pub const UniqueCameraModel: u16 = 0xc614; //Type.Ascii
pub const CFAPlaneColor: u16 = 0xc616; //Type.Byte
pub const CFALayout: u16 = 0xc617; //Type.Short
pub const LinearizationTable: u16 = 0xc618; //Type.Short
pub const BlackLevelRepeatDim: u16 = 0xc619; //Type.Short
pub const BlackLevel: u16 = 0xc61a; //Type.Short
pub const WhiteLevel: u16 = 0xc61d; //Type.Short
pub const DefaultScale: u16 = 0xc61e; //Type.Rational
pub const DefaultCropOrigin: u16 = 0xc61f; //Type.Long
pub const DefaultCropSize: u16 = 0xc620; //Type.Long
pub const ColorMatrix1: u16 = 0xc621; //Type.Srational
pub const ColorMatrix2: u16 = 0xc622; //Type.Srational
pub const CameraCalibration1: u16 = 0xc623; //Type.Srational
pub const CameraCalibration2: u16 = 0xc624; //Type.Srational
pub const AnalogBalance: u16 = 0xc627; //Type.Rational
pub const AsShotNeutral: u16 = 0xc628; //Type.Rational
pub const BaselineExposure: u16 = 0xc62a; //Type.Srational
pub const BaselineNoise: u16 = 0xc62b; //Type.Rational
pub const BaselineSharpness: u16 = 0xc62c; //Type.Rational
pub const BayerGreenSplit: u16 = 0xc62d; //Type.Long
pub const LinearResponseLimit: u16 = 0xc62e; //Type.Rational
pub const CameraSerialNumber: u16 = 0xc62f; //Type.Ascii
pub const AntiAliasStrength: u16 = 0xc632; //Type.Rational
pub const ShadowScale: u16 = 0xc633; //Type.Rational
pub const DNGPrivateData: u16 = 0xc634; //Type.Byte
pub const MakerNoteSafety: u16 = 0xc635; //Type.Short
pub const CalibrationIlluminant1: u16 = 0xc65a; //Type.Short
pub const CalibrationIlluminant2: u16 = 0xc65b; //Type.Short
pub const BestQualityScale: u16 = 0xc65c; //Type.Rational
pub const RawDataUniqueID: u16 = 0xc65d; //Type.Byte
pub const ActiveArea: u16 = 0xc68d; //Type.Long
pub const CameraCalibrationSignature: u16 = 0xc6f3; //Type.Ascii
pub const ProfileCalibrationSignature: u16 = 0xc6f4; //Type.Ascii
pub const NoiseReductionApplied: u16 = 0xc6f7; //Type.Rational
pub const ProfileName: u16 = 0xc6f8; //Type.Ascii
pub const ProfileHueSatMapDims: u16 = 0xc6f9; //Type.Long
pub const ProfileHueSatMapData1: u16 = 0xc6fa; //Type.Float
pub const ProfileHueSatMapData2: u16 = 0xc6fb; //Type.Float
pub const ProfileEmbedPolicy: u16 = 0xc6fd; //Type.Long
pub const PreviewApplicationName: u16 = 0xc716; //Type.Ascii
pub const PreviewApplicationVersion: u16 = 0xc717; //Type.Ascii
pub const PreviewSettingsDigest: u16 = 0xc719; //Type.Byte
pub const PreviewColorSpace: u16 = 0xc71a; //Type.Long
pub const PreviewDateTime: u16 = 0xc71b; //Type.Ascii
pub const NoiseProfile: u16 = 0xc761; //Type.Double
pub const TimeCodes: u16 = 0xc763; //Type.Byte
pub const FrameRate: u16 = 0xc764; //Type.Srational
pub const OpcodeList1: u16 = 0xc740; //Type.Undefined
pub const OpcodeList2: u16 = 0xc741; //Type.Undefined
pub const ReelName: u16 = 0xc789; //Type.Ascii
pub const BaselineExposureOffset: u16 = 0xc7a5; //Type.Srational
pub const NewRawImageDigest: u16 = 0xc7a7; //Type.Byte


// extracted from file
// exiv2 -P nxy 20191226_170725.dng | grep -v "No XMP" | awk '{print "let "$2": u16 = "$1"; // "$3}'

pub const ExifTag: u16 = 0x8769; // Long
pub const ISOSpeedRatings: u16 = 0x8827; // Short
pub const TIFFEPStandardID: u16 = 0x9216; // Byte
pub const ForwardMatrix1: u16 = 0xc714; // SRational
pub const ForwardMatrix2: u16 = 0xc715; // SRational