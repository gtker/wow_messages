@0xc5e6f2ed6ed76065;

# Generated from intermediate_representation_schema.json (JTD/RFC 8927).
# Note: JTD optional/nullable scalar fields are represented via Maybe* wrappers.

struct IntermediateRepresentation {
  version @0 :SchemaVersion;
  world @1 :Objects;
  integerTypeInformation @2 :List(RootIntegerTypeInformationEntry);
  distinctLoginVersionsOtherThanAll @3 :List(UInt8);
  loginVersionOpcodes @4 :List(RootLoginVersionOpcodesEntry);
  login @5 :Objects;
  vanillaUpdateMask @6 :List(UpdateMask);
  tbcUpdateMask @7 :List(UpdateMask);
  wrathUpdateMask @8 :List(UpdateMask);
}

enum UpdateMaskObjectType {
  object @0;
  item @1;
  unit @2;
  player @3;
  container @4;
  gameObject @5;
  dynamicObject @6;
  corpse @7;
}

struct UpdateMaskDataTypeGuid {
}

struct UpdateMaskDataTypeInt {
}

struct UpdateMaskDataTypeFloat {
}

struct UpdateMaskDataTypeTwoShortContent {
  first @0 :TwoShortType;
  second @1 :TwoShortType;
}

struct UpdateMaskDataTypeTwoShort {
  content @0 :UpdateMaskDataTypeTwoShortContent;
}

struct UpdateMaskDataTypeGuidArrayUsingEnumContent {
  definer @0 :Definer;
  variableName @1 :Text;
}

struct UpdateMaskDataTypeGuidArrayUsingEnum {
  content @0 :UpdateMaskDataTypeGuidArrayUsingEnumContent;
}

struct UpdateMaskDataTypeArrayOfStructContent {
  updateMaskStruct @0 :UpdateMaskStruct;
  size @1 :Int32;
  variableName @2 :Text;
}

struct UpdateMaskDataTypeArrayOfStruct {
  content @0 :UpdateMaskDataTypeArrayOfStructContent;
}

struct UpdateMaskDataTypeBytesContent {
  first @0 :ByteType;
  second @1 :ByteType;
  third @2 :ByteType;
  fourth @3 :ByteType;
}

struct UpdateMaskDataTypeBytes {
  content @0 :UpdateMaskDataTypeBytesContent;
}

struct UpdateMaskDataType {
  union {
    guid @0 :UpdateMaskDataTypeGuid;
    int @1 :UpdateMaskDataTypeInt;
    float @2 :UpdateMaskDataTypeFloat;
    twoShort @3 :UpdateMaskDataTypeTwoShort;
    guidArrayUsingEnum @4 :UpdateMaskDataTypeGuidArrayUsingEnum;
    arrayOfStruct @5 :UpdateMaskDataTypeArrayOfStruct;
    bytes @6 :UpdateMaskDataTypeBytes;
  }
}

struct UpdateMask {
  objectType @0 :UpdateMaskObjectType;
  name @1 :Text;
  offset @2 :UInt16;
  size @3 :UInt16;
  dataType @4 :UpdateMaskDataType;
}

struct ByteTypeInnerTypeByte {
}

struct ByteTypeInnerTypeDefiner {
  byteType @0 :Text;
}

struct ByteTypeInnerType {
  union {
    byte @0 :ByteTypeInnerTypeByte;
    definer @1 :ByteTypeInnerTypeDefiner;
  }
}

struct ByteType {
  name @0 :Text;
  innerType @1 :ByteTypeInnerType;
}

struct TwoShortTypeInnerTypeShort {
}

struct TwoShortTypeInnerTypeDefiner {
  twoShortType @0 :Text;
}

struct TwoShortTypeInnerType {
  union {
    short @0 :TwoShortTypeInnerTypeShort;
    definer @1 :TwoShortTypeInnerTypeDefiner;
  }
}

struct TwoShortType {
  name @0 :Text;
  innerType @1 :TwoShortTypeInnerType;
}

struct SchemaVersion {
  major @0 :UInt32;
  minor @1 :UInt32;
  patch @2 :UInt32;
}

struct Objects {
  enums @0 :List(Definer);
  flags @1 :List(Definer);
  structs @2 :List(Container);
  messages @3 :List(Container);
}

struct DefinitionsDefinerObjectsUsedInItemStruct {
  objectName @0 :Text;
  definerUsage @1 :DefinerUsage;
}

struct Definer {
  name @0 :Text;
  definerType @1 :DefinerType;
  objectsUsedIn @2 :List(DefinitionsDefinerObjectsUsedInItemStruct);
  enumerators @3 :List(Enumerator);
  integerType @4 :IntegerType;
  tags @5 :ObjectTags;
  fileInfo @6 :FileInfo;
}

struct Sizes {
  constantSized @0 :Bool;
  minimumSize @1 :UInt32;
  maximumSize @2 :UInt32;
}

struct MaybeUInt16 {
  union {
    none @0 :Void;
    some @1 :UInt16;
  }
}

struct Container {
  name @0 :Text;
  objectType @1 :ObjectType;
  hasManualSizeField @2 :Bool;
  manualSizeSubtraction @3 :MaybeUInt16;
  sizes @4 :Sizes;
  fileInfo @5 :FileInfo;
  onlyHasIoError @6 :Bool;
  tags @7 :ObjectTags;
  members @8 :List(StructMember);
  tests @9 :List(TestCase);
  optional @10 :OptionalMembers;
  preparedObjects @11 :List(PreparedObject);
}

struct DefinitionsPreparedObjectEnumeratorsEntry {
  key @0 :Text;
  value @1 :List(PreparedObject);
}

struct PreparedObject {
  name @0 :Text;
  isElseifFlag @1 :Bool;
  enumPartOfSeparateStatements @2 :Bool;
  enumerators @3 :List(DefinitionsPreparedObjectEnumeratorsEntry);
  definerType @4 :DefinerType;
}

struct DefinitionsUpdateMaskStructMembersItemItemStruct {
  member @0 :Definition;
  offset @1 :Int32;
  size @2 :Int32;
}

struct UpdateMaskStruct {
  name @0 :Text;
  sizes @1 :Sizes;
  fileInfo @2 :FileInfo;
  tags @3 :ObjectTags;
  members @4 :List(List(DefinitionsUpdateMaskStructMembersItemItemStruct));
}

struct FloatingPointValue {
  value @0 :Float64;
  originalString @1 :Text;
}

struct TestCaseValueInteger {
  content @0 :Value;
}

struct TestCaseValueBool {
  content @0 :Bool;
}

struct TestCaseValuePopulation {
  content @0 :Float32;
}

struct TestCaseValueDateTime {
  content @0 :Value;
}

struct TestCaseValueGuid {
  content @0 :Value;
}

struct TestCaseValueFloatingPoint {
  content @0 :FloatingPointValue;
}

struct TestCaseValueArrayContent {
  values @0 :List(Text);
  size @1 :ArraySize;
}

struct TestCaseValueArray {
  content @0 :TestCaseValueArrayContent;
}

struct TestCaseValueString {
  content @0 :Text;
}

struct TestCaseValueFlag {
  content @0 :List(Text);
}

struct TestCaseValueEnum {
  content @0 :Value;
}

struct TestCaseValueSubObjectContent {
  typeName @0 :Text;
  members @1 :List(TestCaseMember);
}

struct TestCaseValueSubObject {
  content @0 :TestCaseValueSubObjectContent;
}

struct TestCaseValueArrayOfSubObjectContent {
  typeName @0 :Text;
  members @1 :List(List(TestCaseMember));
  size @2 :ArraySize;
}

struct TestCaseValueArrayOfSubObject {
  content @0 :TestCaseValueArrayOfSubObjectContent;
}

enum DefinitionsTestCaseValueUpdateMaskContentItemStructUpdateMaskType {
  object @0;
  item @1;
  unit @2;
  player @3;
  container @4;
  gameObject @5;
  dynamicObject @6;
  corpse @7;
}

struct DefinitionsTestCaseValueUpdateMaskContentItemStruct {
  updateMaskType @0 :DefinitionsTestCaseValueUpdateMaskContentItemStructUpdateMaskType;
  updateMaskName @1 :Text;
  updateMaskValue @2 :Text;
}

struct TestCaseValueUpdateMask {
  content @0 :List(DefinitionsTestCaseValueUpdateMaskContentItemStruct);
}

struct TestCaseValueIpAddress {
  content @0 :Value;
}

struct TestCaseValueSeconds {
  content @0 :Value;
}

struct TestCaseValueMilliseconds {
  content @0 :Value;
}

struct TestCaseValueGold {
  content @0 :Value;
}

struct TestCaseValueLevel {
  content @0 :Value;
}

struct DefinitionsTestCaseValueMonsterMoveSplineContentItemStruct {
  x @0 :Float32;
  y @1 :Float32;
  z @2 :Float32;
}

struct TestCaseValueMonsterMoveSpline {
  content @0 :List(DefinitionsTestCaseValueMonsterMoveSplineContentItemStruct);
}

struct TestCaseValue {
  union {
    integer @0 :TestCaseValueInteger;
    bool @1 :TestCaseValueBool;
    population @2 :TestCaseValuePopulation;
    dateTime @3 :TestCaseValueDateTime;
    guid @4 :TestCaseValueGuid;
    floatingPoint @5 :TestCaseValueFloatingPoint;
    array @6 :TestCaseValueArray;
    string @7 :TestCaseValueString;
    flag @8 :TestCaseValueFlag;
    enumField @9 :TestCaseValueEnum;
    subObject @10 :TestCaseValueSubObject;
    arrayOfSubObject @11 :TestCaseValueArrayOfSubObject;
    updateMask @12 :TestCaseValueUpdateMask;
    ipAddress @13 :TestCaseValueIpAddress;
    seconds @14 :TestCaseValueSeconds;
    milliseconds @15 :TestCaseValueMilliseconds;
    gold @16 :TestCaseValueGold;
    level @17 :TestCaseValueLevel;
    monsterMoveSpline @18 :TestCaseValueMonsterMoveSpline;
  }
}

struct TestCaseMember {
  variableName @0 :Text;
  value @1 :TestCaseValue;
  tags @2 :MemberTags;
}

struct TestCase {
  subject @0 :Text;
  members @1 :List(TestCaseMember);
  rawBytes @2 :List(UInt8);
  tags @3 :ObjectTags;
  fileInfo @4 :FileInfo;
}

struct MaybeBool {
  union {
    none @0 :Void;
    some @1 :Bool;
  }
}

struct ObjectTags {
  version @0 :ObjectVersions;
  comment @1 :Text;
  unimplemented @2 :MaybeBool;
  compressed @3 :MaybeBool;
  nonNetworkType @4 :MaybeBool;
  usedInUpdateMask @5 :MaybeBool;
}

struct MemberTagsValidRange {
  fromField @0 :Text;
  to @1 :Text;
}

struct MemberTags {
  comment @0 :Text;
  display @1 :Text;
  maximumLength @2 :Text;
  validRange @3 :MemberTagsValidRange;
}

struct ObjectVersionsLogin {
  versionType @0 :LoginVersions;
}

struct ObjectVersionsWorld {
  versionType @0 :WorldVersions;
}

struct ObjectVersions {
  union {
    login @0 :ObjectVersionsLogin;
    world @1 :ObjectVersionsWorld;
  }
}

struct LoginVersionsSpecific {
  versions @0 :List(UInt8);
}

struct LoginVersionsAll {
}

struct LoginVersions {
  union {
    specific @0 :LoginVersionsSpecific;
    all @1 :LoginVersionsAll;
  }
}

struct WorldVersionsSpecific {
  versions @0 :List(WorldVersion);
}

struct WorldVersionsAll {
}

struct WorldVersions {
  union {
    specific @0 :WorldVersionsSpecific;
    all @1 :WorldVersionsAll;
  }
}

struct MaybeUInt8 {
  union {
    none @0 :Void;
    some @1 :UInt8;
  }
}

struct WorldVersion {
  major @0 :UInt8;
  minor @1 :MaybeUInt8;
  patch @2 :MaybeUInt8;
  build @3 :MaybeUInt16;
}

enum IntegerType {
  u8 @0;
  i8 @1;
  u16 @2;
  u32 @3;
  u64 @4;
  i16 @5;
  i32 @6;
  i64 @7;
  u48 @8;
}

struct FileInfo {
  fileName @0 :Text;
  startPosition @1 :UInt32;
  endPosition @2 :UInt32;
}

struct Value {
  value @0 :Text;
  originalString @1 :Text;
}

enum DefinerType {
  enumField @0;
  flag @1;
}

struct Enumerator {
  name @0 :Text;
  value @1 :Value;
  tags @2 :MemberTags;
}

enum DefinerUsage {
  regularUse @0;
  inIfStatement @1;
}

struct ObjectTypeStruct {
}

struct ObjectTypeCLogin {
  opcode @0 :UInt16;
}

struct ObjectTypeSLogin {
  opcode @0 :UInt16;
}

struct ObjectTypeMsg {
  opcode @0 :UInt16;
}

struct ObjectTypeCMsg {
  opcode @0 :UInt16;
}

struct ObjectTypeSMsg {
  opcode @0 :UInt16;
}

struct ObjectType {
  union {
    structField @0 :ObjectTypeStruct;
    cLogin @1 :ObjectTypeCLogin;
    sLogin @2 :ObjectTypeSLogin;
    msg @3 :ObjectTypeMsg;
    cMsg @4 :ObjectTypeCMsg;
    sMsg @5 :ObjectTypeSMsg;
  }
}

struct StructMemberDefinition {
  structMemberContent @0 :Definition;
}

struct StructMemberIfStatement {
  structMemberContent @0 :IfStatement;
}

struct StructMember {
  union {
    definition @0 :StructMemberDefinition;
    ifStatement @1 :StructMemberIfStatement;
  }
}

struct Definition {
  name @0 :Text;
  dataType @1 :DataType;
  constantValue @2 :Value;
  sizeOfFieldsBeforeSize @3 :MaybeUInt8;
  usedAsSizeIn @4 :Text;
  usedInIf @5 :Bool;
  tags @6 :MemberTags;
}

struct ArrayTypeInteger {
  integerType @0 :IntegerType;
}

struct ArrayTypeStruct {
  structData @0 :Container;
}

struct ArrayTypeCString {
}

struct ArrayTypeGuid {
}

struct ArrayTypePackedGuid {
}

struct ArrayTypeSpell {
}

struct ArrayType {
  union {
    integer @0 :ArrayTypeInteger;
    structField @1 :ArrayTypeStruct;
    cString @2 :ArrayTypeCString;
    guid @3 :ArrayTypeGuid;
    packedGuid @4 :ArrayTypePackedGuid;
    spell @5 :ArrayTypeSpell;
  }
}

struct ArraySizeFixed {
  size @0 :Text;
}

struct ArraySizeVariable {
  size @0 :Text;
}

struct ArraySizeEndless {
}

struct ArraySize {
  union {
    fixedField @0 :ArraySizeFixed;
    variable @1 :ArraySizeVariable;
    endless @2 :ArraySizeEndless;
  }
}

struct DataTypeInteger {
  integerType @0 :IntegerType;
}

struct DataTypeBool {
  integerType @0 :IntegerType;
}

struct DataTypePopulation {
}

struct DataTypeDateTime {
}

struct DataTypePackedGuid {
}

struct DataTypeGuid {
}

struct DataTypeNamedGuid {
}

struct DataTypeFloatingPoint {
}

struct DataTypeCString {
}

struct DataTypeSizedCString {
}

struct DataTypeString {
}

struct DataTypeSpell {
}

struct DataTypeSpell16 {
}

struct DataTypeItem {
}

struct DataTypeArray {
  innerType @0 :ArrayType;
  size @1 :ArraySize;
  compressed @2 :Bool;
}

struct DataTypeEnum {
  typeName @0 :Text;
  integerType @1 :IntegerType;
  tags @2 :ObjectTags;
  upcast @3 :Bool;
}

struct DataTypeFlag {
  typeName @0 :Text;
  integerType @1 :IntegerType;
  tags @2 :ObjectTags;
  upcast @3 :Bool;
}

struct DataTypeStruct {
  structData @0 :Container;
}

struct DataTypeUpdateMask {
}

struct DataTypeAuraMask {
}

struct DataTypeMonsterMoveSpline {
}

struct DataTypeAchievementDoneArray {
}

struct DataTypeAchievementInProgressArray {
}

struct DataTypeEnchantMask {
}

struct DataTypeInspectTalentGearMask {
}

struct DataTypeGold {
}

struct DataTypeLevel {
}

struct DataTypeLevel16 {
}

struct DataTypeLevel32 {
}

struct DataTypeVariableItemRandomProperty {
}

struct DataTypeAddonArray {
}

struct DataTypeIpAddress {
}

struct DataTypeSeconds {
}

struct DataTypeMilliseconds {
}

struct DataTypeCacheMask {
}

struct DataType {
  union {
    integer @0 :DataTypeInteger;
    bool @1 :DataTypeBool;
    population @2 :DataTypePopulation;
    dateTime @3 :DataTypeDateTime;
    packedGuid @4 :DataTypePackedGuid;
    guid @5 :DataTypeGuid;
    namedGuid @6 :DataTypeNamedGuid;
    floatingPoint @7 :DataTypeFloatingPoint;
    cString @8 :DataTypeCString;
    sizedCString @9 :DataTypeSizedCString;
    string @10 :DataTypeString;
    spell @11 :DataTypeSpell;
    spell16 @12 :DataTypeSpell16;
    item @13 :DataTypeItem;
    array @14 :DataTypeArray;
    enumField @15 :DataTypeEnum;
    flag @16 :DataTypeFlag;
    structField @17 :DataTypeStruct;
    updateMask @18 :DataTypeUpdateMask;
    auraMask @19 :DataTypeAuraMask;
    monsterMoveSpline @20 :DataTypeMonsterMoveSpline;
    achievementDoneArray @21 :DataTypeAchievementDoneArray;
    achievementInProgressArray @22 :DataTypeAchievementInProgressArray;
    enchantMask @23 :DataTypeEnchantMask;
    inspectTalentGearMask @24 :DataTypeInspectTalentGearMask;
    gold @25 :DataTypeGold;
    level @26 :DataTypeLevel;
    level16 @27 :DataTypeLevel16;
    level32 @28 :DataTypeLevel32;
    variableItemRandomProperty @29 :DataTypeVariableItemRandomProperty;
    addonArray @30 :DataTypeAddonArray;
    ipAddress @31 :DataTypeIpAddress;
    seconds @32 :DataTypeSeconds;
    milliseconds @33 :DataTypeMilliseconds;
    cacheMask @34 :DataTypeCacheMask;
  }
}

struct IfStatement {
  variableName @0 :Text;
  definerType @1 :DefinerType;
  values @2 :List(Text);
  members @3 :List(StructMember);
  elseIfStatements @4 :List(IfStatement);
  originalType @5 :DataType;
  partOfSeparateIfStatement @6 :Bool;
  isElseIfFlag @7 :Bool;
}

struct OptionalMembers {
  name @0 :Text;
  members @1 :List(StructMember);
  preparedObjects @2 :List(PreparedObject);
}

struct RootIntegerTypeInformationValueStruct {
  size @0 :UInt8;
  bits @1 :UInt8;
}

struct RootIntegerTypeInformationEntry {
  key @0 :Text;
  value @1 :RootIntegerTypeInformationValueStruct;
}

struct RootLoginVersionOpcodesEntry {
  key @0 :Text;
  value @1 :UInt8;
}
