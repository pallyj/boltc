; ModuleID = 'mandlebrot'
source_filename = "mandlebrot"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

%"7runtime6StringS" = type { i8*, i64 }
%"7runtime4Bool8OptionalV" = type { i32, {}, [1 x i8] }
%"7runtime3Int5RangeS" = type { i64, i64 }
%"7runtime3Int11ClosedRangeS" = type { i64, i64 }
%"7runtime3Int8OptionalV" = type { i32, {}, [8 x i8] }
%"7runtime4UInt5RangeS" = type { i64, i64 }
%"7runtime4UInt11ClosedRangeS" = type { i64, i64 }
%"7runtime4UInt8OptionalV" = type { i32, {}, [8 x i8] }
%"7runtime4Int85RangeS" = type { i8, i8 }
%"7runtime4Int811ClosedRangeS" = type { i8, i8 }
%"7runtime4Int88OptionalV" = type { i32, {}, [1 x i8] }
%"7runtime5Int165RangeS" = type { i16, i16 }
%"7runtime5Int1611ClosedRangeS" = type { i16, i16 }
%"7runtime5Int168OptionalV" = type { i32, {}, [2 x i8] }
%"7runtime5Int325RangeS" = type { i32, i32 }
%"7runtime5Int3211ClosedRangeS" = type { i32, i32 }
%"7runtime5Int328OptionalV" = type { i32, {}, [4 x i8] }
%"7runtime5Int645RangeS" = type { i64, i64 }
%"7runtime5Int6411ClosedRangeS" = type { i64, i64 }
%"7runtime5Int648OptionalV" = type { i32, {}, [8 x i8] }
%"7runtime5UInt85RangeS" = type { i8, i8 }
%"7runtime5UInt811ClosedRangeS" = type { i8, i8 }
%"7runtime5UInt88OptionalV" = type { i32, {}, [1 x i8] }
%"7runtime6UInt165RangeS" = type { i16, i16 }
%"7runtime6UInt1611ClosedRangeS" = type { i16, i16 }
%"7runtime6UInt168OptionalV" = type { i32, {}, [2 x i8] }
%"7runtime6UInt325RangeS" = type { i32, i32 }
%"7runtime6UInt3211ClosedRangeS" = type { i32, i32 }
%"7runtime6UInt328OptionalV" = type { i32, {}, [4 x i8] }
%"7runtime6UInt645RangeS" = type { i64, i64 }
%"7runtime6UInt6411ClosedRangeS" = type { i64, i64 }
%"7runtime6UInt648OptionalV" = type { i32, {}, [8 x i8] }
%"7runtime6String8OptionalV" = type { i32, {}, [16 x i8] }
%"7runtime4Char8OptionalV" = type { i32, {}, [4 x i8] }

@gstr = private unnamed_addr constant [3 x i8] c"**\00", align 1
@gstr.1 = private unnamed_addr constant [3 x i8] c"  \00", align 1

declare void @printInt8(i8) local_unnamed_addr

declare void @printUInt8(i8) local_unnamed_addr

declare void @printInt16(i16) local_unnamed_addr

declare void @printUInt16(i16) local_unnamed_addr

declare void @printInt32(i32) local_unnamed_addr

declare void @printUInt32(i32) local_unnamed_addr

declare void @printInt64(i64) local_unnamed_addr

declare void @printUInt64(i64) local_unnamed_addr

declare void @printDouble(double) local_unnamed_addr

declare void @printFloat(float) local_unnamed_addr

declare void @printBool(i1) local_unnamed_addr

declare void @printString(i8*, i64) local_unnamed_addr

declare void @printChar(i32) local_unnamed_addr

declare i32 @readInternalChar() local_unnamed_addr

declare void @printLine() local_unnamed_addr

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define void @.init() local_unnamed_addr #0 {
bb:
  ret void
}

define void @"10mandlebrot4mainFE"() local_unnamed_addr {
bb:
  %_0 = alloca i64, align 8
  %_1 = alloca i64, align 8
  %_4 = alloca i64, align 8
  %_5 = alloca float, align 4
  %_7 = alloca i64, align 8
  %_10 = alloca i64, align 8
  %_13 = alloca i64, align 8
  %_16 = alloca i64, align 8
  %_17 = alloca float, align 4
  %_18 = alloca float, align 4
  %_24 = alloca %"7runtime6StringS", align 8
  %_25 = alloca i1, align 1
  %_27 = alloca %"7runtime6StringS", align 8
  %_28 = alloca i1, align 1
  %_29 = alloca i64, align 8
  %_31 = alloca i64, align 8
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_0, i64 1600)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_1, i64 1)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_4, i64 2)
  %copy = load i64, i64* %_0, align 8
  %copy1 = load i64, i64* %_4, align 8
  %call = tail call i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %copy, i64 %copy1)
  %call3 = tail call i64 @"7runtime3Int6negateF7runtime3IntSE0"(i64 %call)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_7, i64 2)
  %copy5 = load i64, i64* %_7, align 8
  %call6 = tail call i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %copy, i64 %copy5)
  call void @"7runtime5Float4initF7runtime3IntSE0"(float* nonnull %_5, i64 %call6)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_10, i64 2)
  %copy962 = load i64, i64* %_10, align 8
  %call1063 = tail call i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %copy, i64 %copy962)
  %call1364 = tail call i1 @"7runtime3Int8lessThanF7runtime3IntS7runtime3IntSE00"(i64 %call3, i64 %call1063)
  br i1 %call1364, label %bb4.lr.ph, label %bb18

bb4.lr.ph:                                        ; preds = %bb
  %copy30 = load float, float* %_5, align 4
  %copy39 = load i64, i64* %_1, align 8
  %copy44.elt = getelementptr inbounds %"7runtime6StringS", %"7runtime6StringS"* %_24, i64 0, i32 0
  %copy44.elt52 = getelementptr inbounds %"7runtime6StringS", %"7runtime6StringS"* %_24, i64 0, i32 1
  %copy42.elt = getelementptr inbounds %"7runtime6StringS", %"7runtime6StringS"* %_27, i64 0, i32 0
  %copy42.elt55 = getelementptr inbounds %"7runtime6StringS", %"7runtime6StringS"* %_27, i64 0, i32 1
  br label %bb4

bb4:                                              ; preds = %bb4.lr.ph, %bb16
  %_2.065 = phi i64 [ %call3, %bb4.lr.ph ], [ %call51, %bb16 ]
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_13, i64 2)
  %copy16 = load i64, i64* %_13, align 8
  %call17 = tail call i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %copy, i64 %copy16)
  %call19 = tail call i64 @"7runtime3Int6negateF7runtime3IntSE0"(i64 %call17)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_16, i64 2)
  %copy2158 = load i64, i64* %_16, align 8
  %call2259 = tail call i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %copy, i64 %copy2158)
  %call2560 = tail call i1 @"7runtime3Int8lessThanF7runtime3IntS7runtime3IntSE00"(i64 %call19, i64 %call2259)
  br i1 %call2560, label %bb9, label %bb16

bb9:                                              ; preds = %bb4, %bb14
  %_11.061 = phi i64 [ %call48, %bb14 ], [ %call19, %bb4 ]
  call void @"7runtime5Float4initF7runtime3IntSE0"(float* nonnull %_17, i64 %_2.065)
  call void @"7runtime5Float4initF7runtime3IntSE0"(float* nonnull %_18, i64 %_11.061)
  %copy29 = load float, float* %_17, align 4
  %call31 = tail call float @"7runtime5Float3divF7runtime5FloatS7runtime5FloatSE00"(float %copy29, float %copy30)
  %copy32 = load float, float* %_18, align 4
  %call34 = tail call float @"7runtime5Float3divF7runtime5FloatS7runtime5FloatSE00"(float %copy32, float %copy30)
  %call37 = tail call i64 @"10mandlebrot11getDistanceF7runtime5FloatS7runtime5FloatSE00"(float %call31, float %call34)
  %call40 = tail call i1 @"7runtime3Int11greaterThanF7runtime3IntS7runtime3IntSE00"(i64 %call37, i64 %copy39)
  br i1 %call40, label %bb12, label %bb13

bb12:                                             ; preds = %bb9
  call void @"7runtime6String4initFtiE3ptr3len"(%"7runtime6StringS"* nonnull %_27, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @gstr, i64 0, i64 0), i64 2)
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_28, i1 false)
  %copy42.unpack = load i8*, i8** %copy42.elt, align 8
  %0 = insertvalue %"7runtime6StringS" undef, i8* %copy42.unpack, 0
  %copy42.unpack56 = load i64, i64* %copy42.elt55, align 8
  %copy4257 = insertvalue %"7runtime6StringS" %0, i64 %copy42.unpack56, 1
  %copy43 = load i1, i1* %_28, align 1
  tail call void @"7runtime5printF7runtime6StringS7runtime4BoolSE07newline"(%"7runtime6StringS" %copy4257, i1 %copy43)
  br label %bb14

bb13:                                             ; preds = %bb9
  call void @"7runtime6String4initFtiE3ptr3len"(%"7runtime6StringS"* nonnull %_24, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @gstr.1, i64 0, i64 0), i64 2)
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_25, i1 false)
  %copy44.unpack = load i8*, i8** %copy44.elt, align 8
  %1 = insertvalue %"7runtime6StringS" undef, i8* %copy44.unpack, 0
  %copy44.unpack53 = load i64, i64* %copy44.elt52, align 8
  %copy4454 = insertvalue %"7runtime6StringS" %1, i64 %copy44.unpack53, 1
  %copy45 = load i1, i1* %_25, align 1
  tail call void @"7runtime5printF7runtime6StringS7runtime4BoolSE07newline"(%"7runtime6StringS" %copy4454, i1 %copy45)
  br label %bb14

bb14:                                             ; preds = %bb12, %bb13
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_29, i64 1)
  %copy47 = load i64, i64* %_29, align 8
  %call48 = tail call i64 @"7runtime3Int3addF7runtime3IntS7runtime3IntSE00"(i64 %_11.061, i64 %copy47)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_16, i64 2)
  %copy21 = load i64, i64* %_16, align 8
  %call22 = tail call i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %copy, i64 %copy21)
  %call25 = tail call i1 @"7runtime3Int8lessThanF7runtime3IntS7runtime3IntSE00"(i64 %call48, i64 %call22)
  br i1 %call25, label %bb9, label %bb16

bb16:                                             ; preds = %bb14, %bb4
  tail call void @printLine()
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_31, i64 1)
  %copy50 = load i64, i64* %_31, align 8
  %call51 = tail call i64 @"7runtime3Int3addF7runtime3IntS7runtime3IntSE00"(i64 %_2.065, i64 %copy50)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_10, i64 2)
  %copy9 = load i64, i64* %_10, align 8
  %call10 = tail call i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %copy, i64 %copy9)
  %call13 = tail call i1 @"7runtime3Int8lessThanF7runtime3IntS7runtime3IntSE00"(i64 %call51, i64 %call10)
  br i1 %call13, label %bb4, label %bb18

bb18:                                             ; preds = %bb16, %bb
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind writeonly
define i64 @"10mandlebrot11getDistanceF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #1 {
bb:
  %_4 = alloca float, align 4
  %_7 = alloca float, align 4
  %_9 = alloca { float, float }, align 8
  %_11 = alloca { float, float }, align 8
  %_12 = alloca i64, align 8
  %_14 = alloca i64, align 8
  %_22 = alloca float, align 4
  %_24 = alloca i64, align 8
  %_25 = alloca i64, align 8
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_4, float 5.000000e-01)
  %copy3 = load float, float* %_4, align 4
  %call = tail call float @"7runtime5Float3subF7runtime5FloatS7runtime5FloatSE00"(float %1, float %copy3)
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_7, float 5.000000e-01)
  %copy10 = load float, float* %_7, align 4
  %call11 = tail call float @"7runtime5Float3subF7runtime5FloatS7runtime5FloatSE00"(float %1, float %copy10)
  %tuple.item14 = getelementptr inbounds { float, float }, { float, float }* %_9, i64 0, i32 0
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %tuple.item14, float 0.000000e+00)
  %tuple.item15 = getelementptr inbounds { float, float }, { float, float }* %_9, i64 0, i32 1
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %tuple.item15, float 0.000000e+00)
  %copy17 = load float, float* %tuple.item14, align 8
  %tuple.item18 = getelementptr inbounds { float, float }, { float, float }* %_11, i64 0, i32 0
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %tuple.item18, float 0.000000e+00)
  %tuple.item19 = getelementptr inbounds { float, float }, { float, float }* %_11, i64 0, i32 1
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %tuple.item19, float 0.000000e+00)
  %copy21 = load float, float* %tuple.item19, align 4
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_12, i64 0)
  %_12.promoted = load i64, i64* %_12, align 8
  br label %bb2

bb2:                                              ; preds = %bb5, %bb
  %call2461 = phi i64 [ %_12.promoted, %bb ], [ %call24, %bb5 ]
  %_8.0 = phi float [ %copy17, %bb ], [ %call45, %bb5 ]
  %_10.0 = phi float [ %copy21, %bb ], [ %call39, %bb5 ]
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_14, i64 1)
  %copy23 = load i64, i64* %_14, align 8
  %call24 = tail call i64 @"7runtime3Int3addF7runtime3IntS7runtime3IntSE00"(i64 %call2461, i64 %copy23)
  %call27 = tail call float @"7runtime5Float3mulF7runtime5FloatS7runtime5FloatSE00"(float %_10.0, float %_8.0)
  %call30 = tail call float @"7runtime5Float3mulF7runtime5FloatS7runtime5FloatSE00"(float %_10.0, float %_10.0)
  %call33 = tail call float @"7runtime5Float3mulF7runtime5FloatS7runtime5FloatSE00"(float %_8.0, float %_8.0)
  %call36 = tail call float @"7runtime5Float3subF7runtime5FloatS7runtime5FloatSE00"(float %call30, float %call33)
  %call39 = tail call float @"7runtime5Float3addF7runtime5FloatS7runtime5FloatSE00"(float %call36, float %call11)
  %call42 = tail call float @"7runtime5Float3addF7runtime5FloatS7runtime5FloatSE00"(float %call27, float %call27)
  %call45 = tail call float @"7runtime5Float3addF7runtime5FloatS7runtime5FloatSE00"(float %call42, float %0)
  %call48 = tail call float @"7runtime5Float3addF7runtime5FloatS7runtime5FloatSE00"(float %call33, float %call30)
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_22, float 1.600000e+01)
  %copy50 = load float, float* %_22, align 4
  %call51 = tail call i1 @"7runtime5Float11greaterThanF7runtime5FloatS7runtime5FloatSE00"(float %call48, float %copy50)
  br i1 %call51, label %common.ret.loopexit, label %bb5

common.ret.loopexit:                              ; preds = %bb2
  store i64 %call24, i64* %_12, align 8
  br label %common.ret

common.ret:                                       ; preds = %common.ret.loopexit, %bb7
  %common.ret.op = phi i64 [ %copy58, %bb7 ], [ %call24, %common.ret.loopexit ]
  ret i64 %common.ret.op

bb5:                                              ; preds = %bb2
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_24, i64 1000)
  %copy55 = load i64, i64* %_24, align 8
  %call56 = tail call i1 @"7runtime3Int11greaterThanF7runtime3IntS7runtime3IntSE00"(i64 %call24, i64 %copy55)
  br i1 %call56, label %bb7, label %bb2

bb7:                                              ; preds = %bb5
  store i64 %call24, i64* %_12, align 8
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_25, i64 0)
  %copy58 = load i64, i64* %_25, align 8
  br label %common.ret
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Bool3andF7runtime4BoolS7runtime4BoolSE00"(i1 %0, i1 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %iand = and i1 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %iand)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Bool2orF7runtime4BoolS7runtime4BoolSE00"(i1 %0, i1 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %ior = or i1 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %ior)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Bool6invertF7runtime4BoolSE0"(i1 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i1, align 1
  %iinv = xor i1 %0, true
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_1, i1 %iinv)
  %copy2 = load i1, i1* %_1, align 1
  ret i1 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Bool5equalF7runtime4BoolS7runtime4BoolSE00"(i1 %0, i1 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %2 = xor i1 %0, %1
  %eq = xor i1 %2, true
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Bool8notEqualF7runtime4BoolS7runtime4BoolSE00"(i1 %0, i1 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = xor i1 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Bool4initFbE4repr"(i1* nocapture writeonly %0, i1 %1) local_unnamed_addr #2 {
bb:
  store i1 %1, i1* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i1 @"7runtime4Bool8Optional6unwrapF7runtime4BoolSE6orElse"(%"7runtime4Bool8OptionalV" %0, i1 %1) local_unnamed_addr #0 {
bb:
  %.fca.0.extract = extractvalue %"7runtime4Bool8OptionalV" %0, 0
  %.fca.2.0.extract = extractvalue %"7runtime4Bool8OptionalV" %0, 2, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  %2 = and i8 %.fca.2.0.extract, 1
  %3 = icmp ne i8 %2, 0
  %_2.0 = select i1 %cond, i1 %3, i1 %1
  ret i1 %_2.0
}

define %"7runtime4Bool8OptionalV" @"7runtime4Bool8Optional3mapFF7runtime4BoolSE7runtime4BoolSE0"(%"7runtime4Bool8OptionalV" %0, i1 (i1)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime4Bool8OptionalV" %0, 0
  %_3.sroa.2 = alloca i8, align 4
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.0.extract = extractvalue %"7runtime4Bool8OptionalV" %0, 2, 0
  %2 = and i8 %.fca.2.0.extract, 1
  %3 = icmp ne i8 %2, 0
  %call = tail call i1 %1(i1 %3)
  %_3.sroa.2.0.sroa_cast12 = bitcast i8* %_3.sroa.2 to i1*
  store i1 %call, i1* %_3.sroa.2.0.sroa_cast12, align 4
  %_3.sroa.2.0._3.sroa.2.0._3.sroa.2.4.copy8.fca.2.0.load = load i8, i8* %_3.sroa.2, align 4
  %copy8.fca.2.0.insert = insertvalue %"7runtime4Bool8OptionalV" { i32 1, {} poison, [1 x i8] poison }, i8 %_3.sroa.2.0._3.sroa.2.0._3.sroa.2.4.copy8.fca.2.0.load, 2, 0
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4Bool8OptionalV" [ %copy8.fca.2.0.insert, %bb3 ], [ { i32 0, {} poison, [1 x i8] undef }, %bb ]
  ret %"7runtime4Bool8OptionalV" %_2.0
}

define %"7runtime4Bool8OptionalV" @"7runtime4Bool8Optional7flatMapFF7runtime4BoolSE7runtime4Bool8OptionalVE0"(%"7runtime4Bool8OptionalV" %0, %"7runtime4Bool8OptionalV" (i1)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime4Bool8OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.0.extract = extractvalue %"7runtime4Bool8OptionalV" %0, 2, 0
  %2 = and i8 %.fca.2.0.extract, 1
  %3 = icmp ne i8 %2, 0
  %call = tail call %"7runtime4Bool8OptionalV" %1(i1 %3)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4Bool8OptionalV" [ %call, %bb3 ], [ { i32 0, {} poison, [1 x i8] undef }, %bb ]
  ret %"7runtime4Bool8OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define half @"7runtime4Half3addF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca half, align 2
  %fadd = fadd half %0, %1
  call void @"7runtime4Half4initFhE4repr"(half* nonnull %_2, half %fadd)
  %copy3 = load half, half* %_2, align 2
  ret half %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define half @"7runtime4Half3subF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca half, align 2
  %fsub = fsub half %0, %1
  call void @"7runtime4Half4initFhE4repr"(half* nonnull %_2, half %fsub)
  %copy3 = load half, half* %_2, align 2
  ret half %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define half @"7runtime4Half3mulF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca half, align 2
  %fmul = fmul half %0, %1
  call void @"7runtime4Half4initFhE4repr"(half* nonnull %_2, half %fmul)
  %copy3 = load half, half* %_2, align 2
  ret half %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define half @"7runtime4Half3divF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca half, align 2
  %fdiv = fdiv half %0, %1
  call void @"7runtime4Half4initFhE4repr"(half* nonnull %_2, half %fdiv)
  %copy3 = load half, half* %_2, align 2
  ret half %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define half @"7runtime4Half3modF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca half, align 2
  %frem = frem half %0, %1
  call void @"7runtime4Half4initFhE4repr"(half* nonnull %_2, half %frem)
  %copy3 = load half, half* %_2, align 2
  ret half %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Half5equalF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %feq = fcmp oeq half %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %feq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Half8notEqualF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fneq = fcmp one half %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fneq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Half8lessThanF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %flt = fcmp olt half %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %flt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Half11greaterThanF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fgt = fcmp ogt half %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fgt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Half10lessThanEqF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %flte = fcmp ole half %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %flte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Half13greaterThanEqF7runtime4HalfS7runtime4HalfSE00"(half %0, half %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fgte = fcmp oge half %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fgte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define half @"7runtime4Half6negateF7runtime4HalfSE0"(half %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca half, align 2
  %fneg = fneg half %0
  call void @"7runtime4Half4initFhE4repr"(half* nonnull %_1, half %fneg)
  %copy2 = load half, half* %_1, align 2
  ret half %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define half @"7runtime4Half4unitF7runtime4HalfSE0"(half returned %0) local_unnamed_addr #0 {
bb:
  ret half %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Half4initF7runtime5FloatSE10truncating"(half* nocapture writeonly %0, float %1) local_unnamed_addr #2 {
bb:
  %ftrunc16 = fptrunc float %1 to half
  store half %ftrunc16, half* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Half4initF7runtime6DoubleSE10truncating"(half* nocapture writeonly %0, double %1) local_unnamed_addr #2 {
bb:
  %ftrunc16 = fptrunc double %1 to half
  store half %ftrunc16, half* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Half4initF7runtime6UInt64SE0"(half* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf16 = uitofp i64 %1 to half
  store half %icnvf16, half* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Half4initF7runtime5Int64SE0"(half* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf16 = sitofp i64 %1 to half
  store half %icnvf16, half* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Half4initF7runtime4UIntSE0"(half* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf16 = uitofp i64 %1 to half
  store half %icnvf16, half* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Half4initF7runtime3IntSE0"(half* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf16 = sitofp i64 %1 to half
  store half %icnvf16, half* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Half4initFhE4repr"(half* nocapture writeonly %0, half %1) local_unnamed_addr #2 {
bb:
  store half %1, half* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define float @"7runtime5Float3addF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca float, align 4
  %fadd = fadd float %0, %1
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_2, float %fadd)
  %copy3 = load float, float* %_2, align 4
  ret float %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define float @"7runtime5Float3subF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca float, align 4
  %fsub = fsub float %0, %1
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_2, float %fsub)
  %copy3 = load float, float* %_2, align 4
  ret float %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define float @"7runtime5Float3mulF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca float, align 4
  %fmul = fmul float %0, %1
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_2, float %fmul)
  %copy3 = load float, float* %_2, align 4
  ret float %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define float @"7runtime5Float3divF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca float, align 4
  %fdiv = fdiv float %0, %1
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_2, float %fdiv)
  %copy3 = load float, float* %_2, align 4
  ret float %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define float @"7runtime5Float3modF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca float, align 4
  %frem = frem float %0, %1
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_2, float %frem)
  %copy3 = load float, float* %_2, align 4
  ret float %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Float5equalF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %feq = fcmp oeq float %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %feq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Float8notEqualF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fneq = fcmp one float %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fneq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Float8lessThanF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %flt = fcmp olt float %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %flt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Float11greaterThanF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fgt = fcmp ogt float %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fgt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Float10lessThanEqF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %flte = fcmp ole float %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %flte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Float13greaterThanEqF7runtime5FloatS7runtime5FloatSE00"(float %0, float %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fgte = fcmp oge float %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fgte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define float @"7runtime5Float6negateF7runtime5FloatSE0"(float %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca float, align 4
  %fneg = fneg float %0
  call void @"7runtime5Float4initFfE4repr"(float* nonnull %_1, float %fneg)
  %copy2 = load float, float* %_1, align 4
  ret float %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define float @"7runtime5Float4unitF7runtime5FloatSE0"(float returned %0) local_unnamed_addr #0 {
bb:
  ret float %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Float4initF7runtime4HalfSE0"(float* nocapture writeonly %0, half %1) local_unnamed_addr #2 {
bb:
  %fext32 = fpext half %1 to float
  store float %fext32, float* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Float4initF7runtime6DoubleSE10truncating"(float* nocapture writeonly %0, double %1) local_unnamed_addr #2 {
bb:
  %ftrunc32 = fptrunc double %1 to float
  store float %ftrunc32, float* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Float4initF7runtime6UInt64SE0"(float* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf32 = uitofp i64 %1 to float
  store float %icnvf32, float* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Float4initF7runtime5Int64SE0"(float* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf32 = sitofp i64 %1 to float
  store float %icnvf32, float* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Float4initF7runtime4UIntSE0"(float* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf32 = uitofp i64 %1 to float
  store float %icnvf32, float* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Float4initF7runtime3IntSE0"(float* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf32 = sitofp i64 %1 to float
  store float %icnvf32, float* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Float4initFfE4repr"(float* nocapture writeonly %0, float %1) local_unnamed_addr #2 {
bb:
  store float %1, float* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define double @"7runtime6Double3addF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca double, align 8
  %fadd = fadd double %0, %1
  call void @"7runtime6Double4initFdE4repr"(double* nonnull %_2, double %fadd)
  %copy3 = load double, double* %_2, align 8
  ret double %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define double @"7runtime6Double3subF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca double, align 8
  %fsub = fsub double %0, %1
  call void @"7runtime6Double4initFdE4repr"(double* nonnull %_2, double %fsub)
  %copy3 = load double, double* %_2, align 8
  ret double %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define double @"7runtime6Double3mulF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca double, align 8
  %fmul = fmul double %0, %1
  call void @"7runtime6Double4initFdE4repr"(double* nonnull %_2, double %fmul)
  %copy3 = load double, double* %_2, align 8
  ret double %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define double @"7runtime6Double3divF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca double, align 8
  %fdiv = fdiv double %0, %1
  call void @"7runtime6Double4initFdE4repr"(double* nonnull %_2, double %fdiv)
  %copy3 = load double, double* %_2, align 8
  ret double %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define double @"7runtime6Double3modF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca double, align 8
  %frem = frem double %0, %1
  call void @"7runtime6Double4initFdE4repr"(double* nonnull %_2, double %frem)
  %copy3 = load double, double* %_2, align 8
  ret double %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6Double5equalF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %feq = fcmp oeq double %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %feq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6Double8notEqualF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fneq = fcmp one double %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fneq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6Double8lessThanF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %flt = fcmp olt double %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %flt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6Double11greaterThanF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fgt = fcmp ogt double %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fgt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6Double10lessThanEqF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %flte = fcmp ole double %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %flte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6Double13greaterThanEqF7runtime6DoubleS7runtime6DoubleSE00"(double %0, double %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %fgte = fcmp oge double %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %fgte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define double @"7runtime6Double6negateF7runtime6DoubleSE0"(double %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca double, align 8
  %fneg = fneg double %0
  call void @"7runtime6Double4initFdE4repr"(double* nonnull %_1, double %fneg)
  %copy2 = load double, double* %_1, align 8
  ret double %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define double @"7runtime6Double4unitF7runtime6DoubleSE0"(double returned %0) local_unnamed_addr #0 {
bb:
  ret double %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6Double4initF7runtime4HalfSE0"(double* nocapture writeonly %0, half %1) local_unnamed_addr #2 {
bb:
  %fext64 = fpext half %1 to double
  store double %fext64, double* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6Double4initF7runtime5FloatSE0"(double* nocapture writeonly %0, float %1) local_unnamed_addr #2 {
bb:
  %fext64 = fpext float %1 to double
  store double %fext64, double* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6Double4initF7runtime6UInt64SE0"(double* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf64 = uitofp i64 %1 to double
  store double %icnvf64, double* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6Double4initF7runtime5Int64SE0"(double* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf64 = sitofp i64 %1 to double
  store double %icnvf64, double* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6Double4initF7runtime4UIntSE0"(double* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf64 = uitofp i64 %1 to double
  store double %icnvf64, double* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6Double4initF7runtime3IntSE0"(double* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %icnvf64 = sitofp i64 %1 to double
  store double %icnvf64, double* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6Double4initFdE4repr"(double* nocapture writeonly %0, double %1) local_unnamed_addr #2 {
bb:
  store double %1, double* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int3addF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iadd = add i64 %1, %0
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %iadd)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int3subF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %isub = sub i64 %0, %1
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %isub)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int3mulF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %imul = mul i64 %1, %0
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %imul)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int3divF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime3Int5equalF7runtime3IntS7runtime3IntSE00"(i64 %1, i64 %copy1)
  %idiv = sdiv i64 %0, %1
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_4, i64 %idiv)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int3modF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime3Int5equalF7runtime3IntS7runtime3IntSE00"(i64 %1, i64 %copy1)
  %irem = srem i64 %0, %1
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_4, i64 %irem)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int5bitOrF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ior = or i64 %1, %0
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %ior)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int6bitXorF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ixor = xor i64 %1, %0
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %ixor)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int6bitAndF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iand = and i64 %1, %0
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %iand)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int9shiftLeftF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishl = shl i64 %0, %1
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %ishl)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int10shiftRightF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishr = ashr i64 %0, %1
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_2, i64 %ishr)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime3Int5equalF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime3Int8notEqualF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime3Int8lessThanF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp slt i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime3Int11greaterThanF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp sgt i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime3Int10lessThanEqF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp sle i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime3Int13greaterThanEqF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp sge i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime3Int4unitF7runtime3IntSE0"(i64 returned %0) local_unnamed_addr #0 {
bb:
  ret i64 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime3Int5RangeS" @"7runtime3Int9openRangeF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime3Int5RangeS", align 8
  call void @"7runtime3Int5Range4initF7runtime3IntS7runtime3IntSE4from2to"(%"7runtime3Int5RangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime3Int5RangeS", %"7runtime3Int5RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime3Int5RangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime3Int5RangeS", %"7runtime3Int5RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime3Int5RangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime3Int5RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime3Int11ClosedRangeS" @"7runtime3Int11closedRangeF7runtime3IntS7runtime3IntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime3Int11ClosedRangeS", align 8
  call void @"7runtime3Int11ClosedRange4initF7runtime3IntS7runtime3IntSE4from2to"(%"7runtime3Int11ClosedRangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime3Int11ClosedRangeS", %"7runtime3Int11ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime3Int11ClosedRangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime3Int11ClosedRangeS", %"7runtime3Int11ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime3Int11ClosedRangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime3Int11ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int6negateF7runtime3IntSE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %ineg = sub i64 0, %0
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_1, i64 %ineg)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime3Int6invertF7runtime3IntSE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %iinv = xor i64 %0, -1
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_1, i64 %iinv)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime5FloatSE5floor"(i64* nocapture writeonly %0, float %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi float %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime6DoubleSE5floor"(i64* nocapture writeonly %0, double %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi double %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime4HalfSE5floor"(i64* nocapture writeonly %0, half %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi half %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime5UInt8SE0"(i64* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i8 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime4Int8SE0"(i64* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext64 = sext i8 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime6UInt16SE0"(i64* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i16 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime5Int16SE0"(i64* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext64 = sext i16 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime6UInt32SE0"(i64* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i32 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime5Int32SE0"(i64* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %izext64 = sext i32 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime6UInt64SE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime4UIntSE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initF7runtime5Int64SE0"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int4initFiE4repr"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int11ClosedRange4initF7runtime3IntS7runtime3IntSE4from2to"(%"7runtime3Int11ClosedRangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime3Int11ClosedRangeS", %"7runtime3Int11ClosedRangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime3Int11ClosedRangeS", %"7runtime3Int11ClosedRangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime3Int11ClosedRange6bottomFE"(%"7runtime3Int11ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime3Int11ClosedRangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime3Int11ClosedRange3topFE"(%"7runtime3Int11ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime3Int11ClosedRangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime3Int5Range4initF7runtime3IntS7runtime3IntSE4from2to"(%"7runtime3Int5RangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime3Int5RangeS", %"7runtime3Int5RangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime3Int5RangeS", %"7runtime3Int5RangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime3Int5Range6bottomFE"(%"7runtime3Int5RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime3Int5RangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime3Int5Range3topFE"(%"7runtime3Int5RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime3Int5RangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime3Int8Optional6unwrapF7runtime3IntSE6orElse"(%"7runtime3Int8OptionalV" %0, i64 %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime3Int8OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime3Int8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt7.elt21 = extractvalue [8 x i8] %.elt7, 7
  %.elt7.elt19 = extractvalue [8 x i8] %.elt7, 6
  %.elt7.elt17 = extractvalue [8 x i8] %.elt7, 5
  %.elt7.elt15 = extractvalue [8 x i8] %.elt7, 4
  %.elt7.elt13 = extractvalue [8 x i8] %.elt7, 3
  %.elt7.elt11 = extractvalue [8 x i8] %.elt7, 2
  %.elt7.elt9 = extractvalue [8 x i8] %.elt7, 1
  %.elt7.elt = extractvalue [8 x i8] %.elt7, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt7.elt21 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt7.elt19 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt7.elt17 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt7.elt15 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt7.elt13 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt7.elt11 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt7.elt9 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi i64 [ %_0.sroa.2.4.insert.insert, %bb3 ], [ %1, %bb ]
  ret i64 %_2.0
}

define %"7runtime3Int8OptionalV" @"7runtime3Int8Optional3mapFF7runtime3IntSE7runtime3IntSE0"(%"7runtime3Int8OptionalV" %0, i64 (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime3Int8OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime3Int8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt29 = extractvalue [8 x i8] %.elt15, 7
  %.elt15.elt27 = extractvalue [8 x i8] %.elt15, 6
  %.elt15.elt25 = extractvalue [8 x i8] %.elt15, 5
  %.elt15.elt23 = extractvalue [8 x i8] %.elt15, 4
  %.elt15.elt21 = extractvalue [8 x i8] %.elt15, 3
  %.elt15.elt19 = extractvalue [8 x i8] %.elt15, 2
  %.elt15.elt17 = extractvalue [8 x i8] %.elt15, 1
  %.elt15.elt = extractvalue [8 x i8] %.elt15, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt15.elt29 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt15.elt27 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt15.elt25 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt15.elt23 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt15.elt21 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt15.elt19 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt15.elt17 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call i64 %1(i64 %_0.sroa.2.4.insert.insert)
  %_3.sroa.1.4.extract.trunc = trunc i64 %call to i8
  %_3.sroa.3.4.extract.shift = lshr i64 %call, 8
  %_3.sroa.3.4.extract.trunc = trunc i64 %_3.sroa.3.4.extract.shift to i8
  %_3.sroa.4.4.extract.shift = lshr i64 %call, 16
  %_3.sroa.4.4.extract.trunc = trunc i64 %_3.sroa.4.4.extract.shift to i8
  %_3.sroa.5.4.extract.shift = lshr i64 %call, 24
  %_3.sroa.5.4.extract.trunc = trunc i64 %_3.sroa.5.4.extract.shift to i8
  %_3.sroa.6.4.extract.shift = lshr i64 %call, 32
  %_3.sroa.6.4.extract.trunc = trunc i64 %_3.sroa.6.4.extract.shift to i8
  %_3.sroa.7.4.extract.shift = lshr i64 %call, 40
  %_3.sroa.7.4.extract.trunc = trunc i64 %_3.sroa.7.4.extract.shift to i8
  %_3.sroa.8.4.extract.shift = lshr i64 %call, 48
  %_3.sroa.8.4.extract.trunc = trunc i64 %_3.sroa.8.4.extract.shift to i8
  %_3.sroa.9.4.extract.shift = lshr i64 %call, 56
  %_3.sroa.9.4.extract.trunc = trunc i64 %_3.sroa.9.4.extract.shift to i8
  %2 = insertvalue [8 x i8] undef, i8 %_3.sroa.1.4.extract.trunc, 0
  %3 = insertvalue [8 x i8] %2, i8 %_3.sroa.3.4.extract.trunc, 1
  %4 = insertvalue [8 x i8] %3, i8 %_3.sroa.4.4.extract.trunc, 2
  %5 = insertvalue [8 x i8] %4, i8 %_3.sroa.5.4.extract.trunc, 3
  %6 = insertvalue [8 x i8] %5, i8 %_3.sroa.6.4.extract.trunc, 4
  %7 = insertvalue [8 x i8] %6, i8 %_3.sroa.7.4.extract.trunc, 5
  %8 = insertvalue [8 x i8] %7, i8 %_3.sroa.8.4.extract.trunc, 6
  %copy8.unpack5369 = insertvalue [8 x i8] %8, i8 %_3.sroa.9.4.extract.trunc, 7
  %copy854 = insertvalue %"7runtime3Int8OptionalV" { i32 1, {} undef, [8 x i8] undef }, [8 x i8] %copy8.unpack5369, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime3Int8OptionalV" [ %copy854, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime3Int8OptionalV" %_2.0
}

define %"7runtime3Int8OptionalV" @"7runtime3Int8Optional7flatMapFF7runtime3IntSE7runtime3Int8OptionalVE0"(%"7runtime3Int8OptionalV" %0, %"7runtime3Int8OptionalV" (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime3Int8OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime3Int8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt24 = extractvalue [8 x i8] %.elt10, 7
  %.elt10.elt22 = extractvalue [8 x i8] %.elt10, 6
  %.elt10.elt20 = extractvalue [8 x i8] %.elt10, 5
  %.elt10.elt18 = extractvalue [8 x i8] %.elt10, 4
  %.elt10.elt16 = extractvalue [8 x i8] %.elt10, 3
  %.elt10.elt14 = extractvalue [8 x i8] %.elt10, 2
  %.elt10.elt12 = extractvalue [8 x i8] %.elt10, 1
  %.elt10.elt = extractvalue [8 x i8] %.elt10, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt10.elt24 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt10.elt22 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt10.elt20 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt10.elt18 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt10.elt16 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt10.elt14 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt10.elt12 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime3Int8OptionalV" %1(i64 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime3Int8OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime3Int8OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt3addF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iadd = add i64 %1, %0
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %iadd)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt3subF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %isub = sub i64 %0, %1
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %isub)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt3mulF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %imul = mul i64 %1, %0
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %imul)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt3divF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime4UInt5equalF7runtime4UIntS7runtime4UIntSE00"(i64 %1, i64 %copy1)
  %idiv = udiv i64 %0, %1
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_4, i64 %idiv)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt3modF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime4UInt5equalF7runtime4UIntS7runtime4UIntSE00"(i64 %1, i64 %copy1)
  %irem = urem i64 %0, %1
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_4, i64 %irem)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt5bitOrF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ior = or i64 %1, %0
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %ior)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt6bitXorF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ixor = xor i64 %1, %0
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %ixor)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt6bitAndF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iand = and i64 %1, %0
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %iand)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt9shiftLeftF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishl = shl i64 %0, %1
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %ishl)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt10shiftRightF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishr = lshr i64 %0, %1
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_2, i64 %ishr)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4UInt5equalF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4UInt8notEqualF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4UInt8lessThanF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp ult i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4UInt11greaterThanF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp ugt i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4UInt10lessThanEqF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp ule i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4UInt13greaterThanEqF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp uge i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime4UInt4unitF7runtime4UIntSE0"(i64 returned %0) local_unnamed_addr #0 {
bb:
  ret i64 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime4UInt5RangeS" @"7runtime4UInt9openRangeF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime4UInt5RangeS", align 8
  call void @"7runtime4UInt5Range4initF7runtime4UIntS7runtime4UIntSE4from2to"(%"7runtime4UInt5RangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime4UInt5RangeS", %"7runtime4UInt5RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime4UInt5RangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime4UInt5RangeS", %"7runtime4UInt5RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime4UInt5RangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime4UInt5RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime4UInt11ClosedRangeS" @"7runtime4UInt11closedRangeF7runtime4UIntS7runtime4UIntSE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime4UInt11ClosedRangeS", align 8
  call void @"7runtime4UInt11ClosedRange4initF7runtime4UIntS7runtime4UIntSE4from2to"(%"7runtime4UInt11ClosedRangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime4UInt11ClosedRangeS", %"7runtime4UInt11ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime4UInt11ClosedRangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime4UInt11ClosedRangeS", %"7runtime4UInt11ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime4UInt11ClosedRangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime4UInt11ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt6negateF7runtime4UIntSE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %ineg = sub i64 0, %0
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_1, i64 %ineg)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime4UInt6invertF7runtime4UIntSE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %iinv = xor i64 %0, -1
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_1, i64 %iinv)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime5FloatSE5floor"(i64* nocapture writeonly %0, float %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi float %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime6DoubleSE5floor"(i64* nocapture writeonly %0, double %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi double %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime4HalfSE5floor"(i64* nocapture writeonly %0, half %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi half %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime5UInt8SE0"(i64* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i8 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime6UInt16SE0"(i64* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i16 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime6UInt32SE0"(i64* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i32 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime5Int64SE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime3IntSE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initF7runtime6UInt64SE0"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt4initFiE4repr"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt11ClosedRange4initF7runtime4UIntS7runtime4UIntSE4from2to"(%"7runtime4UInt11ClosedRangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime4UInt11ClosedRangeS", %"7runtime4UInt11ClosedRangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime4UInt11ClosedRangeS", %"7runtime4UInt11ClosedRangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime4UInt11ClosedRange6bottomFE"(%"7runtime4UInt11ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime4UInt11ClosedRangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime4UInt11ClosedRange3topFE"(%"7runtime4UInt11ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime4UInt11ClosedRangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4UInt5Range4initF7runtime4UIntS7runtime4UIntSE4from2to"(%"7runtime4UInt5RangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime4UInt5RangeS", %"7runtime4UInt5RangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime4UInt5RangeS", %"7runtime4UInt5RangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime4UInt5Range6bottomFE"(%"7runtime4UInt5RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime4UInt5RangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime4UInt5Range3topFE"(%"7runtime4UInt5RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime4UInt5RangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime4UInt8Optional6unwrapF7runtime4UIntSE6orElse"(%"7runtime4UInt8OptionalV" %0, i64 %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime4UInt8OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime4UInt8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt7.elt21 = extractvalue [8 x i8] %.elt7, 7
  %.elt7.elt19 = extractvalue [8 x i8] %.elt7, 6
  %.elt7.elt17 = extractvalue [8 x i8] %.elt7, 5
  %.elt7.elt15 = extractvalue [8 x i8] %.elt7, 4
  %.elt7.elt13 = extractvalue [8 x i8] %.elt7, 3
  %.elt7.elt11 = extractvalue [8 x i8] %.elt7, 2
  %.elt7.elt9 = extractvalue [8 x i8] %.elt7, 1
  %.elt7.elt = extractvalue [8 x i8] %.elt7, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt7.elt21 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt7.elt19 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt7.elt17 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt7.elt15 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt7.elt13 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt7.elt11 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt7.elt9 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi i64 [ %_0.sroa.2.4.insert.insert, %bb3 ], [ %1, %bb ]
  ret i64 %_2.0
}

define %"7runtime4UInt8OptionalV" @"7runtime4UInt8Optional3mapFF7runtime4UIntSE7runtime4UIntSE0"(%"7runtime4UInt8OptionalV" %0, i64 (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime4UInt8OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime4UInt8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt29 = extractvalue [8 x i8] %.elt15, 7
  %.elt15.elt27 = extractvalue [8 x i8] %.elt15, 6
  %.elt15.elt25 = extractvalue [8 x i8] %.elt15, 5
  %.elt15.elt23 = extractvalue [8 x i8] %.elt15, 4
  %.elt15.elt21 = extractvalue [8 x i8] %.elt15, 3
  %.elt15.elt19 = extractvalue [8 x i8] %.elt15, 2
  %.elt15.elt17 = extractvalue [8 x i8] %.elt15, 1
  %.elt15.elt = extractvalue [8 x i8] %.elt15, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt15.elt29 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt15.elt27 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt15.elt25 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt15.elt23 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt15.elt21 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt15.elt19 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt15.elt17 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call i64 %1(i64 %_0.sroa.2.4.insert.insert)
  %_3.sroa.1.4.extract.trunc = trunc i64 %call to i8
  %_3.sroa.3.4.extract.shift = lshr i64 %call, 8
  %_3.sroa.3.4.extract.trunc = trunc i64 %_3.sroa.3.4.extract.shift to i8
  %_3.sroa.4.4.extract.shift = lshr i64 %call, 16
  %_3.sroa.4.4.extract.trunc = trunc i64 %_3.sroa.4.4.extract.shift to i8
  %_3.sroa.5.4.extract.shift = lshr i64 %call, 24
  %_3.sroa.5.4.extract.trunc = trunc i64 %_3.sroa.5.4.extract.shift to i8
  %_3.sroa.6.4.extract.shift = lshr i64 %call, 32
  %_3.sroa.6.4.extract.trunc = trunc i64 %_3.sroa.6.4.extract.shift to i8
  %_3.sroa.7.4.extract.shift = lshr i64 %call, 40
  %_3.sroa.7.4.extract.trunc = trunc i64 %_3.sroa.7.4.extract.shift to i8
  %_3.sroa.8.4.extract.shift = lshr i64 %call, 48
  %_3.sroa.8.4.extract.trunc = trunc i64 %_3.sroa.8.4.extract.shift to i8
  %_3.sroa.9.4.extract.shift = lshr i64 %call, 56
  %_3.sroa.9.4.extract.trunc = trunc i64 %_3.sroa.9.4.extract.shift to i8
  %2 = insertvalue [8 x i8] undef, i8 %_3.sroa.1.4.extract.trunc, 0
  %3 = insertvalue [8 x i8] %2, i8 %_3.sroa.3.4.extract.trunc, 1
  %4 = insertvalue [8 x i8] %3, i8 %_3.sroa.4.4.extract.trunc, 2
  %5 = insertvalue [8 x i8] %4, i8 %_3.sroa.5.4.extract.trunc, 3
  %6 = insertvalue [8 x i8] %5, i8 %_3.sroa.6.4.extract.trunc, 4
  %7 = insertvalue [8 x i8] %6, i8 %_3.sroa.7.4.extract.trunc, 5
  %8 = insertvalue [8 x i8] %7, i8 %_3.sroa.8.4.extract.trunc, 6
  %copy8.unpack5369 = insertvalue [8 x i8] %8, i8 %_3.sroa.9.4.extract.trunc, 7
  %copy854 = insertvalue %"7runtime4UInt8OptionalV" { i32 1, {} undef, [8 x i8] undef }, [8 x i8] %copy8.unpack5369, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4UInt8OptionalV" [ %copy854, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime4UInt8OptionalV" %_2.0
}

define %"7runtime4UInt8OptionalV" @"7runtime4UInt8Optional7flatMapFF7runtime4UIntSE7runtime4UInt8OptionalVE0"(%"7runtime4UInt8OptionalV" %0, %"7runtime4UInt8OptionalV" (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime4UInt8OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime4UInt8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt24 = extractvalue [8 x i8] %.elt10, 7
  %.elt10.elt22 = extractvalue [8 x i8] %.elt10, 6
  %.elt10.elt20 = extractvalue [8 x i8] %.elt10, 5
  %.elt10.elt18 = extractvalue [8 x i8] %.elt10, 4
  %.elt10.elt16 = extractvalue [8 x i8] %.elt10, 3
  %.elt10.elt14 = extractvalue [8 x i8] %.elt10, 2
  %.elt10.elt12 = extractvalue [8 x i8] %.elt10, 1
  %.elt10.elt = extractvalue [8 x i8] %.elt10, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt10.elt24 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt10.elt22 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt10.elt20 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt10.elt18 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt10.elt16 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt10.elt14 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt10.elt12 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime4UInt8OptionalV" %1(i64 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4UInt8OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime4UInt8OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int83addF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %iadd = add i8 %1, %0
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %iadd)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int83subF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %isub = sub i8 %0, %1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %isub)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int83mulF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %imul = mul i8 %1, %0
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %imul)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int83divF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i8, align 1
  %_4 = alloca i8, align 1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_3, i8 0)
  %copy1 = load i8, i8* %_3, align 1
  %call = tail call i1 @"7runtime4Int85equalF7runtime4Int8S7runtime4Int8SE00"(i8 %1, i8 %copy1)
  %idiv = sdiv i8 %0, %1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_4, i8 %idiv)
  %copy6 = load i8, i8* %_4, align 1
  ret i8 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int83modF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i8, align 1
  %_4 = alloca i8, align 1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_3, i8 0)
  %copy1 = load i8, i8* %_3, align 1
  %call = tail call i1 @"7runtime4Int85equalF7runtime4Int8S7runtime4Int8SE00"(i8 %1, i8 %copy1)
  %irem = srem i8 %0, %1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_4, i8 %irem)
  %copy6 = load i8, i8* %_4, align 1
  ret i8 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int85bitOrF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ior = or i8 %1, %0
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %ior)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int86bitXorF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ixor = xor i8 %1, %0
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %ixor)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int86bitAndF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %iand = and i8 %1, %0
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %iand)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int89shiftLeftF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ishl = shl i8 %0, %1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %ishl)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int810shiftRightF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ishr = ashr i8 %0, %1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_2, i8 %ishr)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Int85equalF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Int88notEqualF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Int88lessThanF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp slt i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Int811greaterThanF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp sgt i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Int810lessThanEqF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp sle i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Int813greaterThanEqF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp sge i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime4Int84unitF7runtime4Int8SE0"(i8 returned %0) local_unnamed_addr #0 {
bb:
  ret i8 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime4Int85RangeS" @"7runtime4Int89openRangeF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime4Int85RangeS", align 8
  call void @"7runtime4Int85Range4initF7runtime4Int8S7runtime4Int8SE4from2to"(%"7runtime4Int85RangeS"* nonnull %_2, i8 %0, i8 %1)
  %copy2.elt = getelementptr inbounds %"7runtime4Int85RangeS", %"7runtime4Int85RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i8, i8* %copy2.elt, align 8
  %2 = insertvalue %"7runtime4Int85RangeS" undef, i8 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime4Int85RangeS", %"7runtime4Int85RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i8, i8* %copy2.elt3, align 1
  %copy25 = insertvalue %"7runtime4Int85RangeS" %2, i8 %copy2.unpack4, 1
  ret %"7runtime4Int85RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime4Int811ClosedRangeS" @"7runtime4Int811closedRangeF7runtime4Int8S7runtime4Int8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime4Int811ClosedRangeS", align 8
  call void @"7runtime4Int811ClosedRange4initF7runtime4Int8S7runtime4Int8SE4from2to"(%"7runtime4Int811ClosedRangeS"* nonnull %_2, i8 %0, i8 %1)
  %copy2.elt = getelementptr inbounds %"7runtime4Int811ClosedRangeS", %"7runtime4Int811ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i8, i8* %copy2.elt, align 8
  %2 = insertvalue %"7runtime4Int811ClosedRangeS" undef, i8 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime4Int811ClosedRangeS", %"7runtime4Int811ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i8, i8* %copy2.elt3, align 1
  %copy25 = insertvalue %"7runtime4Int811ClosedRangeS" %2, i8 %copy2.unpack4, 1
  ret %"7runtime4Int811ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int86negateF7runtime4Int8SE0"(i8 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i8, align 1
  %ineg = sub i8 0, %0
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_1, i8 %ineg)
  %copy2 = load i8, i8* %_1, align 1
  ret i8 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime4Int86invertF7runtime4Int8SE0"(i8 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i8, align 1
  %iinv = xor i8 %0, -1
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_1, i8 %iinv)
  %copy2 = load i8, i8* %_1, align 1
  ret i8 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Int84initF7runtime5Int16SE10truncating"(i8* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %itrunc8 = trunc i16 %1 to i8
  store i8 %itrunc8, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Int84initF7runtime5Int32SE10truncating"(i8* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %itrunc8 = trunc i32 %1 to i8
  store i8 %itrunc8, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Int84initF7runtime5Int64SE10truncating"(i8* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %itrunc8 = trunc i64 %1 to i8
  store i8 %itrunc8, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Int84initF7runtime5UInt8SE7bitcast"(i8* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  store i8 %1, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Int84initFaE4repr"(i8* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  store i8 %1, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Int811ClosedRange4initF7runtime4Int8S7runtime4Int8SE4from2to"(%"7runtime4Int811ClosedRangeS"* nocapture writeonly %0, i8 %1, i8 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime4Int811ClosedRangeS", %"7runtime4Int811ClosedRangeS"* %0, i64 0, i32 0
  store i8 %1, i8* %gep, align 1
  %gep3 = getelementptr inbounds %"7runtime4Int811ClosedRangeS", %"7runtime4Int811ClosedRangeS"* %0, i64 0, i32 1
  store i8 %2, i8* %gep3, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime4Int811ClosedRange6bottomFE"(%"7runtime4Int811ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime4Int811ClosedRangeS" %0, 0
  ret i8 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime4Int811ClosedRange3topFE"(%"7runtime4Int811ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime4Int811ClosedRangeS" %0, 1
  ret i8 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Int85Range4initF7runtime4Int8S7runtime4Int8SE4from2to"(%"7runtime4Int85RangeS"* nocapture writeonly %0, i8 %1, i8 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime4Int85RangeS", %"7runtime4Int85RangeS"* %0, i64 0, i32 0
  store i8 %1, i8* %gep, align 1
  %gep3 = getelementptr inbounds %"7runtime4Int85RangeS", %"7runtime4Int85RangeS"* %0, i64 0, i32 1
  store i8 %2, i8* %gep3, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime4Int85Range6bottomFE"(%"7runtime4Int85RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime4Int85RangeS" %0, 0
  ret i8 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime4Int85Range3topFE"(%"7runtime4Int85RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime4Int85RangeS" %0, 1
  ret i8 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime4Int88Optional6unwrapF7runtime4Int8SE6orElse"(%"7runtime4Int88OptionalV" %0, i8 %1) local_unnamed_addr #0 {
bb:
  %.fca.0.extract = extractvalue %"7runtime4Int88OptionalV" %0, 0
  %.fca.2.0.extract = extractvalue %"7runtime4Int88OptionalV" %0, 2, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  %_2.0 = select i1 %cond, i8 %.fca.2.0.extract, i8 %1
  ret i8 %_2.0
}

define %"7runtime4Int88OptionalV" @"7runtime4Int88Optional3mapFF7runtime4Int8SE7runtime4Int8SE0"(%"7runtime4Int88OptionalV" %0, i8 (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime4Int88OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.0.extract = extractvalue %"7runtime4Int88OptionalV" %0, 2, 0
  %call = tail call i8 %1(i8 %.fca.2.0.extract)
  %copy8.fca.2.0.insert = insertvalue %"7runtime4Int88OptionalV" { i32 1, {} poison, [1 x i8] poison }, i8 %call, 2, 0
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4Int88OptionalV" [ %copy8.fca.2.0.insert, %bb3 ], [ { i32 0, {} poison, [1 x i8] undef }, %bb ]
  ret %"7runtime4Int88OptionalV" %_2.0
}

define %"7runtime4Int88OptionalV" @"7runtime4Int88Optional7flatMapFF7runtime4Int8SE7runtime4Int88OptionalVE0"(%"7runtime4Int88OptionalV" %0, %"7runtime4Int88OptionalV" (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime4Int88OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.0.extract = extractvalue %"7runtime4Int88OptionalV" %0, 2, 0
  %call = tail call %"7runtime4Int88OptionalV" %1(i8 %.fca.2.0.extract)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4Int88OptionalV" [ %call, %bb3 ], [ { i32 0, {} poison, [1 x i8] undef }, %bb ]
  ret %"7runtime4Int88OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int163addF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %iadd = add i16 %1, %0
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %iadd)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int163subF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %isub = sub i16 %0, %1
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %isub)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int163mulF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %imul = mul i16 %1, %0
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %imul)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int163divF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i16, align 2
  %_4 = alloca i16, align 2
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_3, i16 0)
  %copy1 = load i16, i16* %_3, align 2
  %call = tail call i1 @"7runtime5Int165equalF7runtime5Int16S7runtime5Int16SE00"(i16 %1, i16 %copy1)
  %idiv = sdiv i16 %0, %1
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_4, i16 %idiv)
  %copy6 = load i16, i16* %_4, align 2
  ret i16 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int163modF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i16, align 2
  %_4 = alloca i16, align 2
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_3, i16 0)
  %copy1 = load i16, i16* %_3, align 2
  %call = tail call i1 @"7runtime5Int165equalF7runtime5Int16S7runtime5Int16SE00"(i16 %1, i16 %copy1)
  %irem = srem i16 %0, %1
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_4, i16 %irem)
  %copy6 = load i16, i16* %_4, align 2
  ret i16 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int165bitOrF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ior = or i16 %1, %0
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %ior)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int166bitXorF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ixor = xor i16 %1, %0
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %ixor)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int166bitAndF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %iand = and i16 %1, %0
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %iand)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int169shiftLeftF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ishl = shl i16 %0, %1
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %ishl)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int1610shiftRightF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ishr = ashr i16 %0, %1
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_2, i16 %ishr)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int165equalF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int168notEqualF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int168lessThanF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp slt i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int1611greaterThanF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp sgt i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int1610lessThanEqF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp sle i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int1613greaterThanEqF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp sge i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime5Int164unitF7runtime5Int16SE0"(i16 returned %0) local_unnamed_addr #0 {
bb:
  ret i16 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5Int165RangeS" @"7runtime5Int169openRangeF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5Int165RangeS", align 8
  call void @"7runtime5Int165Range4initF7runtime5Int16S7runtime5Int16SE4from2to"(%"7runtime5Int165RangeS"* nonnull %_2, i16 %0, i16 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5Int165RangeS", %"7runtime5Int165RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i16, i16* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5Int165RangeS" undef, i16 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5Int165RangeS", %"7runtime5Int165RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i16, i16* %copy2.elt3, align 2
  %copy25 = insertvalue %"7runtime5Int165RangeS" %2, i16 %copy2.unpack4, 1
  ret %"7runtime5Int165RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5Int1611ClosedRangeS" @"7runtime5Int1611closedRangeF7runtime5Int16S7runtime5Int16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5Int1611ClosedRangeS", align 8
  call void @"7runtime5Int1611ClosedRange4initF7runtime5Int16S7runtime5Int16SE4from2to"(%"7runtime5Int1611ClosedRangeS"* nonnull %_2, i16 %0, i16 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5Int1611ClosedRangeS", %"7runtime5Int1611ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i16, i16* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5Int1611ClosedRangeS" undef, i16 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5Int1611ClosedRangeS", %"7runtime5Int1611ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i16, i16* %copy2.elt3, align 2
  %copy25 = insertvalue %"7runtime5Int1611ClosedRangeS" %2, i16 %copy2.unpack4, 1
  ret %"7runtime5Int1611ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int166negateF7runtime5Int16SE0"(i16 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i16, align 2
  %ineg = sub i16 0, %0
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_1, i16 %ineg)
  %copy2 = load i16, i16* %_1, align 2
  ret i16 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime5Int166invertF7runtime5Int16SE0"(i16 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i16, align 2
  %iinv = xor i16 %0, -1
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_1, i16 %iinv)
  %copy2 = load i16, i16* %_1, align 2
  ret i16 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int164initF7runtime5UInt8SE0"(i16* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext16 = zext i8 %1 to i16
  store i16 %izext16, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int164initF7runtime4Int8SE0"(i16* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext16 = sext i8 %1 to i16
  store i16 %izext16, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int164initF7runtime5Int32SE10truncating"(i16* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %itrunc16 = trunc i32 %1 to i16
  store i16 %itrunc16, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int164initF7runtime5Int64SE10truncating"(i16* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %itrunc16 = trunc i64 %1 to i16
  store i16 %itrunc16, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int164initF7runtime6UInt16SE7bitcast"(i16* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  store i16 %1, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int164initFlE4repr"(i16* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  store i16 %1, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int1611ClosedRange4initF7runtime5Int16S7runtime5Int16SE4from2to"(%"7runtime5Int1611ClosedRangeS"* nocapture writeonly %0, i16 %1, i16 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5Int1611ClosedRangeS", %"7runtime5Int1611ClosedRangeS"* %0, i64 0, i32 0
  store i16 %1, i16* %gep, align 2
  %gep3 = getelementptr inbounds %"7runtime5Int1611ClosedRangeS", %"7runtime5Int1611ClosedRangeS"* %0, i64 0, i32 1
  store i16 %2, i16* %gep3, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime5Int1611ClosedRange6bottomFE"(%"7runtime5Int1611ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int1611ClosedRangeS" %0, 0
  ret i16 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime5Int1611ClosedRange3topFE"(%"7runtime5Int1611ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5Int1611ClosedRangeS" %0, 1
  ret i16 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int165Range4initF7runtime5Int16S7runtime5Int16SE4from2to"(%"7runtime5Int165RangeS"* nocapture writeonly %0, i16 %1, i16 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5Int165RangeS", %"7runtime5Int165RangeS"* %0, i64 0, i32 0
  store i16 %1, i16* %gep, align 2
  %gep3 = getelementptr inbounds %"7runtime5Int165RangeS", %"7runtime5Int165RangeS"* %0, i64 0, i32 1
  store i16 %2, i16* %gep3, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime5Int165Range6bottomFE"(%"7runtime5Int165RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int165RangeS" %0, 0
  ret i16 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime5Int165Range3topFE"(%"7runtime5Int165RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5Int165RangeS" %0, 1
  ret i16 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime5Int168Optional6unwrapF7runtime5Int16SE6orElse"(%"7runtime5Int168OptionalV" %0, i16 %1) local_unnamed_addr #0 {
bb:
  %.fca.0.extract = extractvalue %"7runtime5Int168OptionalV" %0, 0
  %.fca.2.0.extract = extractvalue %"7runtime5Int168OptionalV" %0, 2, 0
  %.fca.2.1.extract = extractvalue %"7runtime5Int168OptionalV" %0, 2, 1
  %cond = icmp eq i32 %.fca.0.extract, 1
  %_0.sroa.4.4.insert.ext = zext i8 %.fca.2.1.extract to i16
  %_0.sroa.4.4.insert.shift = shl nuw i16 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.2.4.insert.ext = zext i8 %.fca.2.0.extract to i16
  %_0.sroa.2.4.insert.insert = or i16 %_0.sroa.4.4.insert.shift, %_0.sroa.2.4.insert.ext
  %_2.0 = select i1 %cond, i16 %_0.sroa.2.4.insert.insert, i16 %1
  ret i16 %_2.0
}

define %"7runtime5Int168OptionalV" @"7runtime5Int168Optional3mapFF7runtime5Int16SE7runtime5Int16SE0"(%"7runtime5Int168OptionalV" %0, i16 (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime5Int168OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.1.extract = extractvalue %"7runtime5Int168OptionalV" %0, 2, 1
  %.fca.2.0.extract = extractvalue %"7runtime5Int168OptionalV" %0, 2, 0
  %_0.sroa.4.4.insert.ext = zext i8 %.fca.2.1.extract to i16
  %_0.sroa.4.4.insert.shift = shl nuw i16 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.2.4.insert.ext = zext i8 %.fca.2.0.extract to i16
  %_0.sroa.2.4.insert.insert = or i16 %_0.sroa.4.4.insert.shift, %_0.sroa.2.4.insert.ext
  %call = tail call i16 %1(i16 %_0.sroa.2.4.insert.insert)
  %_3.sroa.2.4.extract.trunc = trunc i16 %call to i8
  %_3.sroa.4.4.extract.shift = lshr i16 %call, 8
  %_3.sroa.4.4.extract.trunc = trunc i16 %_3.sroa.4.4.extract.shift to i8
  %copy8.fca.2.0.insert = insertvalue %"7runtime5Int168OptionalV" { i32 1, {} poison, [2 x i8] poison }, i8 %_3.sroa.2.4.extract.trunc, 2, 0
  %copy8.fca.2.1.insert = insertvalue %"7runtime5Int168OptionalV" %copy8.fca.2.0.insert, i8 %_3.sroa.4.4.extract.trunc, 2, 1
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5Int168OptionalV" [ %copy8.fca.2.1.insert, %bb3 ], [ { i32 0, {} poison, [2 x i8] undef }, %bb ]
  ret %"7runtime5Int168OptionalV" %_2.0
}

define %"7runtime5Int168OptionalV" @"7runtime5Int168Optional7flatMapFF7runtime5Int16SE7runtime5Int168OptionalVE0"(%"7runtime5Int168OptionalV" %0, %"7runtime5Int168OptionalV" (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime5Int168OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.1.extract = extractvalue %"7runtime5Int168OptionalV" %0, 2, 1
  %.fca.2.0.extract = extractvalue %"7runtime5Int168OptionalV" %0, 2, 0
  %_0.sroa.4.4.insert.ext = zext i8 %.fca.2.1.extract to i16
  %_0.sroa.4.4.insert.shift = shl nuw i16 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.2.4.insert.ext = zext i8 %.fca.2.0.extract to i16
  %_0.sroa.2.4.insert.insert = or i16 %_0.sroa.4.4.insert.shift, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime5Int168OptionalV" %1(i16 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5Int168OptionalV" [ %call, %bb3 ], [ { i32 0, {} poison, [2 x i8] undef }, %bb ]
  ret %"7runtime5Int168OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int323addF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %iadd = add i32 %1, %0
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %iadd)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int323subF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %isub = sub i32 %0, %1
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %isub)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int323mulF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %imul = mul i32 %1, %0
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %imul)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int323divF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_4 = alloca i32, align 4
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_3, i32 0)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime5Int325equalF7runtime5Int32S7runtime5Int32SE00"(i32 %1, i32 %copy1)
  %idiv = sdiv i32 %0, %1
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_4, i32 %idiv)
  %copy6 = load i32, i32* %_4, align 4
  ret i32 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int323modF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_4 = alloca i32, align 4
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_3, i32 0)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime5Int325equalF7runtime5Int32S7runtime5Int32SE00"(i32 %1, i32 %copy1)
  %irem = srem i32 %0, %1
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_4, i32 %irem)
  %copy6 = load i32, i32* %_4, align 4
  ret i32 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int325bitOrF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ior = or i32 %1, %0
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %ior)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int326bitXorF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ixor = xor i32 %1, %0
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %ixor)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int326bitAndF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %iand = and i32 %1, %0
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %iand)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int329shiftLeftF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ishl = shl i32 %0, %1
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %ishl)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int3210shiftRightF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ishr = ashr i32 %0, %1
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_2, i32 %ishr)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int325equalF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int328notEqualF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int328lessThanF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp slt i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int3211greaterThanF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp sgt i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int3210lessThanEqF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp sle i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int3213greaterThanEqF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp sge i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime5Int324unitF7runtime5Int32SE0"(i32 returned %0) local_unnamed_addr #0 {
bb:
  ret i32 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5Int325RangeS" @"7runtime5Int329openRangeF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5Int325RangeS", align 8
  call void @"7runtime5Int325Range4initF7runtime5Int32S7runtime5Int32SE4from2to"(%"7runtime5Int325RangeS"* nonnull %_2, i32 %0, i32 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5Int325RangeS", %"7runtime5Int325RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i32, i32* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5Int325RangeS" undef, i32 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5Int325RangeS", %"7runtime5Int325RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i32, i32* %copy2.elt3, align 4
  %copy25 = insertvalue %"7runtime5Int325RangeS" %2, i32 %copy2.unpack4, 1
  ret %"7runtime5Int325RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5Int3211ClosedRangeS" @"7runtime5Int3211closedRangeF7runtime5Int32S7runtime5Int32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5Int3211ClosedRangeS", align 8
  call void @"7runtime5Int3211ClosedRange4initF7runtime5Int32S7runtime5Int32SE4from2to"(%"7runtime5Int3211ClosedRangeS"* nonnull %_2, i32 %0, i32 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5Int3211ClosedRangeS", %"7runtime5Int3211ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i32, i32* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5Int3211ClosedRangeS" undef, i32 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5Int3211ClosedRangeS", %"7runtime5Int3211ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i32, i32* %copy2.elt3, align 4
  %copy25 = insertvalue %"7runtime5Int3211ClosedRangeS" %2, i32 %copy2.unpack4, 1
  ret %"7runtime5Int3211ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int326negateF7runtime5Int32SE0"(i32 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i32, align 4
  %ineg = sub i32 0, %0
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_1, i32 %ineg)
  %copy2 = load i32, i32* %_1, align 4
  ret i32 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime5Int326invertF7runtime5Int32SE0"(i32 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i32, align 4
  %iinv = xor i32 %0, -1
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_1, i32 %iinv)
  %copy2 = load i32, i32* %_1, align 4
  ret i32 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int324initF7runtime5UInt8SE0"(i32* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext32 = zext i8 %1 to i32
  store i32 %izext32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int324initF7runtime4Int8SE0"(i32* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext32 = sext i8 %1 to i32
  store i32 %izext32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int324initF7runtime6UInt16SE0"(i32* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext32 = zext i16 %1 to i32
  store i32 %izext32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int324initF7runtime5Int16SE0"(i32* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext32 = sext i16 %1 to i32
  store i32 %izext32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int324initF7runtime5Int64SE10truncating"(i32* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %itrunc32 = trunc i64 %1 to i32
  store i32 %itrunc32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int324initF7runtime6UInt32SE7bitcast"(i32* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  store i32 %1, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int324initFjE4repr"(i32* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  store i32 %1, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int3211ClosedRange4initF7runtime5Int32S7runtime5Int32SE4from2to"(%"7runtime5Int3211ClosedRangeS"* nocapture writeonly %0, i32 %1, i32 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5Int3211ClosedRangeS", %"7runtime5Int3211ClosedRangeS"* %0, i64 0, i32 0
  store i32 %1, i32* %gep, align 4
  %gep3 = getelementptr inbounds %"7runtime5Int3211ClosedRangeS", %"7runtime5Int3211ClosedRangeS"* %0, i64 0, i32 1
  store i32 %2, i32* %gep3, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime5Int3211ClosedRange6bottomFE"(%"7runtime5Int3211ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int3211ClosedRangeS" %0, 0
  ret i32 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime5Int3211ClosedRange3topFE"(%"7runtime5Int3211ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5Int3211ClosedRangeS" %0, 1
  ret i32 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int325Range4initF7runtime5Int32S7runtime5Int32SE4from2to"(%"7runtime5Int325RangeS"* nocapture writeonly %0, i32 %1, i32 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5Int325RangeS", %"7runtime5Int325RangeS"* %0, i64 0, i32 0
  store i32 %1, i32* %gep, align 4
  %gep3 = getelementptr inbounds %"7runtime5Int325RangeS", %"7runtime5Int325RangeS"* %0, i64 0, i32 1
  store i32 %2, i32* %gep3, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime5Int325Range6bottomFE"(%"7runtime5Int325RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int325RangeS" %0, 0
  ret i32 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime5Int325Range3topFE"(%"7runtime5Int325RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5Int325RangeS" %0, 1
  ret i32 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime5Int328Optional6unwrapF7runtime5Int32SE6orElse"(%"7runtime5Int328OptionalV" %0, i32 %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int328OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime5Int328OptionalV" %0, 2
  %.elt7.elt = extractvalue [4 x i8] %.elt7, 0
  %.elt7.elt9 = extractvalue [4 x i8] %.elt7, 1
  %.elt7.elt11 = extractvalue [4 x i8] %.elt7, 2
  %.elt7.elt13 = extractvalue [4 x i8] %.elt7, 3
  %cond = icmp eq i32 %.elt, 1
  %_0.sroa.6.4.insert.ext = zext i8 %.elt7.elt13 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt7.elt11 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt7.elt9 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %_2.0 = select i1 %cond, i32 %_0.sroa.2.4.insert.insert, i32 %1
  ret i32 %_2.0
}

define %"7runtime5Int328OptionalV" @"7runtime5Int328Optional3mapFF7runtime5Int32SE7runtime5Int32SE0"(%"7runtime5Int328OptionalV" %0, i32 (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime5Int328OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime5Int328OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt21 = extractvalue [4 x i8] %.elt15, 3
  %.elt15.elt19 = extractvalue [4 x i8] %.elt15, 2
  %.elt15.elt17 = extractvalue [4 x i8] %.elt15, 1
  %.elt15.elt = extractvalue [4 x i8] %.elt15, 0
  %_0.sroa.6.4.insert.ext = zext i8 %.elt15.elt21 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt15.elt19 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt15.elt17 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %call = tail call i32 %1(i32 %_0.sroa.2.4.insert.insert)
  %_3.sroa.1.4.extract.trunc = trunc i32 %call to i8
  %_3.sroa.3.4.extract.shift = lshr i32 %call, 8
  %_3.sroa.3.4.extract.trunc = trunc i32 %_3.sroa.3.4.extract.shift to i8
  %_3.sroa.4.4.extract.shift = lshr i32 %call, 16
  %_3.sroa.4.4.extract.trunc = trunc i32 %_3.sroa.4.4.extract.shift to i8
  %_3.sroa.5.4.extract.shift = lshr i32 %call, 24
  %_3.sroa.5.4.extract.trunc = trunc i32 %_3.sroa.5.4.extract.shift to i8
  %2 = insertvalue [4 x i8] undef, i8 %_3.sroa.1.4.extract.trunc, 0
  %3 = insertvalue [4 x i8] %2, i8 %_3.sroa.3.4.extract.trunc, 1
  %4 = insertvalue [4 x i8] %3, i8 %_3.sroa.4.4.extract.trunc, 2
  %copy8.unpack3745 = insertvalue [4 x i8] %4, i8 %_3.sroa.5.4.extract.trunc, 3
  %copy838 = insertvalue %"7runtime5Int328OptionalV" { i32 1, {} undef, [4 x i8] undef }, [4 x i8] %copy8.unpack3745, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5Int328OptionalV" [ %copy838, %bb3 ], [ { i32 0, {} undef, [4 x i8] undef }, %bb ]
  ret %"7runtime5Int328OptionalV" %_2.0
}

define %"7runtime5Int328OptionalV" @"7runtime5Int328Optional7flatMapFF7runtime5Int32SE7runtime5Int328OptionalVE0"(%"7runtime5Int328OptionalV" %0, %"7runtime5Int328OptionalV" (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime5Int328OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime5Int328OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt16 = extractvalue [4 x i8] %.elt10, 3
  %.elt10.elt14 = extractvalue [4 x i8] %.elt10, 2
  %.elt10.elt12 = extractvalue [4 x i8] %.elt10, 1
  %.elt10.elt = extractvalue [4 x i8] %.elt10, 0
  %_0.sroa.6.4.insert.ext = zext i8 %.elt10.elt16 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt10.elt14 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt10.elt12 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime5Int328OptionalV" %1(i32 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5Int328OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [4 x i8] undef }, %bb ]
  ret %"7runtime5Int328OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int643addF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iadd = add i64 %1, %0
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %iadd)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int643subF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %isub = sub i64 %0, %1
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %isub)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int643mulF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %imul = mul i64 %1, %0
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %imul)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int643divF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime5Int645equalF7runtime5Int64S7runtime5Int64SE00"(i64 %1, i64 %copy1)
  %idiv = sdiv i64 %0, %1
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_4, i64 %idiv)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int643modF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime5Int645equalF7runtime5Int64S7runtime5Int64SE00"(i64 %1, i64 %copy1)
  %irem = srem i64 %0, %1
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_4, i64 %irem)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int645bitOrF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ior = or i64 %1, %0
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %ior)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int646bitXorF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ixor = xor i64 %1, %0
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %ixor)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int646bitAndF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iand = and i64 %1, %0
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %iand)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int649shiftLeftF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishl = shl i64 %0, %1
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %ishl)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int6410shiftRightF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishr = ashr i64 %0, %1
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_2, i64 %ishr)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int645equalF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int648notEqualF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int648lessThanF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp slt i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int6411greaterThanF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp sgt i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int6410lessThanEqF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp sle i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5Int6413greaterThanEqF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp sge i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime5Int644unitF7runtime5Int64SE0"(i64 returned %0) local_unnamed_addr #0 {
bb:
  ret i64 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5Int645RangeS" @"7runtime5Int649openRangeF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5Int645RangeS", align 8
  call void @"7runtime5Int645Range4initF7runtime5Int64S7runtime5Int64SE4from2to"(%"7runtime5Int645RangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5Int645RangeS", %"7runtime5Int645RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5Int645RangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5Int645RangeS", %"7runtime5Int645RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime5Int645RangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime5Int645RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5Int6411ClosedRangeS" @"7runtime5Int6411closedRangeF7runtime5Int64S7runtime5Int64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5Int6411ClosedRangeS", align 8
  call void @"7runtime5Int6411ClosedRange4initF7runtime5Int64S7runtime5Int64SE4from2to"(%"7runtime5Int6411ClosedRangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5Int6411ClosedRangeS", %"7runtime5Int6411ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5Int6411ClosedRangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5Int6411ClosedRangeS", %"7runtime5Int6411ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime5Int6411ClosedRangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime5Int6411ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int646negateF7runtime5Int64SE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %ineg = sub i64 0, %0
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_1, i64 %ineg)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime5Int646invertF7runtime5Int64SE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %iinv = xor i64 %0, -1
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_1, i64 %iinv)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime5FloatSE5floor"(i64* nocapture writeonly %0, float %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi float %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime6DoubleSE5floor"(i64* nocapture writeonly %0, double %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi double %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime4HalfSE5floor"(i64* nocapture writeonly %0, half %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi half %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime5UInt8SE0"(i64* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i8 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime4Int8SE0"(i64* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext64 = sext i8 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime6UInt16SE0"(i64* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i16 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime5Int16SE0"(i64* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext64 = sext i16 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime6UInt32SE0"(i64* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i32 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime5Int32SE0"(i64* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %izext64 = sext i32 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime6UInt64SE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime4UIntSE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initF7runtime3IntSE0"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int644initFiE4repr"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int6411ClosedRange4initF7runtime5Int64S7runtime5Int64SE4from2to"(%"7runtime5Int6411ClosedRangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5Int6411ClosedRangeS", %"7runtime5Int6411ClosedRangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime5Int6411ClosedRangeS", %"7runtime5Int6411ClosedRangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime5Int6411ClosedRange6bottomFE"(%"7runtime5Int6411ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int6411ClosedRangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime5Int6411ClosedRange3topFE"(%"7runtime5Int6411ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5Int6411ClosedRangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5Int645Range4initF7runtime5Int64S7runtime5Int64SE4from2to"(%"7runtime5Int645RangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5Int645RangeS", %"7runtime5Int645RangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime5Int645RangeS", %"7runtime5Int645RangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime5Int645Range6bottomFE"(%"7runtime5Int645RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int645RangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime5Int645Range3topFE"(%"7runtime5Int645RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5Int645RangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime5Int648Optional6unwrapF7runtime5Int64SE6orElse"(%"7runtime5Int648OptionalV" %0, i64 %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5Int648OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime5Int648OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt7.elt21 = extractvalue [8 x i8] %.elt7, 7
  %.elt7.elt19 = extractvalue [8 x i8] %.elt7, 6
  %.elt7.elt17 = extractvalue [8 x i8] %.elt7, 5
  %.elt7.elt15 = extractvalue [8 x i8] %.elt7, 4
  %.elt7.elt13 = extractvalue [8 x i8] %.elt7, 3
  %.elt7.elt11 = extractvalue [8 x i8] %.elt7, 2
  %.elt7.elt9 = extractvalue [8 x i8] %.elt7, 1
  %.elt7.elt = extractvalue [8 x i8] %.elt7, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt7.elt21 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt7.elt19 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt7.elt17 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt7.elt15 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt7.elt13 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt7.elt11 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt7.elt9 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi i64 [ %_0.sroa.2.4.insert.insert, %bb3 ], [ %1, %bb ]
  ret i64 %_2.0
}

define %"7runtime5Int648OptionalV" @"7runtime5Int648Optional3mapFF7runtime5Int64SE7runtime5Int64SE0"(%"7runtime5Int648OptionalV" %0, i64 (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime5Int648OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime5Int648OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt29 = extractvalue [8 x i8] %.elt15, 7
  %.elt15.elt27 = extractvalue [8 x i8] %.elt15, 6
  %.elt15.elt25 = extractvalue [8 x i8] %.elt15, 5
  %.elt15.elt23 = extractvalue [8 x i8] %.elt15, 4
  %.elt15.elt21 = extractvalue [8 x i8] %.elt15, 3
  %.elt15.elt19 = extractvalue [8 x i8] %.elt15, 2
  %.elt15.elt17 = extractvalue [8 x i8] %.elt15, 1
  %.elt15.elt = extractvalue [8 x i8] %.elt15, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt15.elt29 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt15.elt27 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt15.elt25 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt15.elt23 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt15.elt21 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt15.elt19 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt15.elt17 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call i64 %1(i64 %_0.sroa.2.4.insert.insert)
  %_3.sroa.1.4.extract.trunc = trunc i64 %call to i8
  %_3.sroa.3.4.extract.shift = lshr i64 %call, 8
  %_3.sroa.3.4.extract.trunc = trunc i64 %_3.sroa.3.4.extract.shift to i8
  %_3.sroa.4.4.extract.shift = lshr i64 %call, 16
  %_3.sroa.4.4.extract.trunc = trunc i64 %_3.sroa.4.4.extract.shift to i8
  %_3.sroa.5.4.extract.shift = lshr i64 %call, 24
  %_3.sroa.5.4.extract.trunc = trunc i64 %_3.sroa.5.4.extract.shift to i8
  %_3.sroa.6.4.extract.shift = lshr i64 %call, 32
  %_3.sroa.6.4.extract.trunc = trunc i64 %_3.sroa.6.4.extract.shift to i8
  %_3.sroa.7.4.extract.shift = lshr i64 %call, 40
  %_3.sroa.7.4.extract.trunc = trunc i64 %_3.sroa.7.4.extract.shift to i8
  %_3.sroa.8.4.extract.shift = lshr i64 %call, 48
  %_3.sroa.8.4.extract.trunc = trunc i64 %_3.sroa.8.4.extract.shift to i8
  %_3.sroa.9.4.extract.shift = lshr i64 %call, 56
  %_3.sroa.9.4.extract.trunc = trunc i64 %_3.sroa.9.4.extract.shift to i8
  %2 = insertvalue [8 x i8] undef, i8 %_3.sroa.1.4.extract.trunc, 0
  %3 = insertvalue [8 x i8] %2, i8 %_3.sroa.3.4.extract.trunc, 1
  %4 = insertvalue [8 x i8] %3, i8 %_3.sroa.4.4.extract.trunc, 2
  %5 = insertvalue [8 x i8] %4, i8 %_3.sroa.5.4.extract.trunc, 3
  %6 = insertvalue [8 x i8] %5, i8 %_3.sroa.6.4.extract.trunc, 4
  %7 = insertvalue [8 x i8] %6, i8 %_3.sroa.7.4.extract.trunc, 5
  %8 = insertvalue [8 x i8] %7, i8 %_3.sroa.8.4.extract.trunc, 6
  %copy8.unpack5369 = insertvalue [8 x i8] %8, i8 %_3.sroa.9.4.extract.trunc, 7
  %copy854 = insertvalue %"7runtime5Int648OptionalV" { i32 1, {} undef, [8 x i8] undef }, [8 x i8] %copy8.unpack5369, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5Int648OptionalV" [ %copy854, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime5Int648OptionalV" %_2.0
}

define %"7runtime5Int648OptionalV" @"7runtime5Int648Optional7flatMapFF7runtime5Int64SE7runtime5Int648OptionalVE0"(%"7runtime5Int648OptionalV" %0, %"7runtime5Int648OptionalV" (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime5Int648OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime5Int648OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt24 = extractvalue [8 x i8] %.elt10, 7
  %.elt10.elt22 = extractvalue [8 x i8] %.elt10, 6
  %.elt10.elt20 = extractvalue [8 x i8] %.elt10, 5
  %.elt10.elt18 = extractvalue [8 x i8] %.elt10, 4
  %.elt10.elt16 = extractvalue [8 x i8] %.elt10, 3
  %.elt10.elt14 = extractvalue [8 x i8] %.elt10, 2
  %.elt10.elt12 = extractvalue [8 x i8] %.elt10, 1
  %.elt10.elt = extractvalue [8 x i8] %.elt10, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt10.elt24 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt10.elt22 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt10.elt20 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt10.elt18 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt10.elt16 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt10.elt14 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt10.elt12 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime5Int648OptionalV" %1(i64 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5Int648OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime5Int648OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt83addF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %iadd = add i8 %1, %0
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %iadd)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt83subF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %isub = sub i8 %0, %1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %isub)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt83mulF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %imul = mul i8 %1, %0
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %imul)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt83divF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i8, align 1
  %_4 = alloca i8, align 1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_3, i8 0)
  %copy1 = load i8, i8* %_3, align 1
  %call = tail call i1 @"7runtime5UInt85equalF7runtime5UInt8S7runtime5UInt8SE00"(i8 %1, i8 %copy1)
  %idiv = udiv i8 %0, %1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_4, i8 %idiv)
  %copy6 = load i8, i8* %_4, align 1
  ret i8 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt83modF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i8, align 1
  %_4 = alloca i8, align 1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_3, i8 0)
  %copy1 = load i8, i8* %_3, align 1
  %call = tail call i1 @"7runtime5UInt85equalF7runtime5UInt8S7runtime5UInt8SE00"(i8 %1, i8 %copy1)
  %irem = urem i8 %0, %1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_4, i8 %irem)
  %copy6 = load i8, i8* %_4, align 1
  ret i8 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt85bitOrF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ior = or i8 %1, %0
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %ior)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt86bitXorF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ixor = xor i8 %1, %0
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %ixor)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt86bitAndF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %iand = and i8 %1, %0
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %iand)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt89shiftLeftF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ishl = shl i8 %0, %1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %ishl)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt810shiftRightF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i8, align 1
  %ishr = lshr i8 %0, %1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_2, i8 %ishr)
  %copy3 = load i8, i8* %_2, align 1
  ret i8 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5UInt85equalF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5UInt88notEqualF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5UInt88lessThanF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp ult i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5UInt811greaterThanF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp ugt i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5UInt810lessThanEqF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp ule i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime5UInt813greaterThanEqF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp uge i8 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime5UInt84unitF7runtime5UInt8SE0"(i8 returned %0) local_unnamed_addr #0 {
bb:
  ret i8 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5UInt85RangeS" @"7runtime5UInt89openRangeF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5UInt85RangeS", align 8
  call void @"7runtime5UInt85Range4initF7runtime5UInt8S7runtime5UInt8SE4from2to"(%"7runtime5UInt85RangeS"* nonnull %_2, i8 %0, i8 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5UInt85RangeS", %"7runtime5UInt85RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i8, i8* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5UInt85RangeS" undef, i8 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5UInt85RangeS", %"7runtime5UInt85RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i8, i8* %copy2.elt3, align 1
  %copy25 = insertvalue %"7runtime5UInt85RangeS" %2, i8 %copy2.unpack4, 1
  ret %"7runtime5UInt85RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime5UInt811ClosedRangeS" @"7runtime5UInt811closedRangeF7runtime5UInt8S7runtime5UInt8SE00"(i8 %0, i8 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime5UInt811ClosedRangeS", align 8
  call void @"7runtime5UInt811ClosedRange4initF7runtime5UInt8S7runtime5UInt8SE4from2to"(%"7runtime5UInt811ClosedRangeS"* nonnull %_2, i8 %0, i8 %1)
  %copy2.elt = getelementptr inbounds %"7runtime5UInt811ClosedRangeS", %"7runtime5UInt811ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i8, i8* %copy2.elt, align 8
  %2 = insertvalue %"7runtime5UInt811ClosedRangeS" undef, i8 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime5UInt811ClosedRangeS", %"7runtime5UInt811ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i8, i8* %copy2.elt3, align 1
  %copy25 = insertvalue %"7runtime5UInt811ClosedRangeS" %2, i8 %copy2.unpack4, 1
  ret %"7runtime5UInt811ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt86negateF7runtime5UInt8SE0"(i8 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i8, align 1
  %ineg = sub i8 0, %0
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_1, i8 %ineg)
  %copy2 = load i8, i8* %_1, align 1
  ret i8 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i8 @"7runtime5UInt86invertF7runtime5UInt8SE0"(i8 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i8, align 1
  %iinv = xor i8 %0, -1
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_1, i8 %iinv)
  %copy2 = load i8, i8* %_1, align 1
  ret i8 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5UInt84initF7runtime6UInt16SE10truncating"(i8* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %itrunc8 = trunc i16 %1 to i8
  store i8 %itrunc8, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5UInt84initF7runtime6UInt32SE10truncating"(i8* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %itrunc8 = trunc i32 %1 to i8
  store i8 %itrunc8, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5UInt84initF7runtime6UInt64SE10truncating"(i8* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %itrunc8 = trunc i64 %1 to i8
  store i8 %itrunc8, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5UInt84initF7runtime4Int8SE7bitcast"(i8* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  store i8 %1, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5UInt84initFaE4repr"(i8* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  store i8 %1, i8* %0, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5UInt811ClosedRange4initF7runtime5UInt8S7runtime5UInt8SE4from2to"(%"7runtime5UInt811ClosedRangeS"* nocapture writeonly %0, i8 %1, i8 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5UInt811ClosedRangeS", %"7runtime5UInt811ClosedRangeS"* %0, i64 0, i32 0
  store i8 %1, i8* %gep, align 1
  %gep3 = getelementptr inbounds %"7runtime5UInt811ClosedRangeS", %"7runtime5UInt811ClosedRangeS"* %0, i64 0, i32 1
  store i8 %2, i8* %gep3, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime5UInt811ClosedRange6bottomFE"(%"7runtime5UInt811ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5UInt811ClosedRangeS" %0, 0
  ret i8 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime5UInt811ClosedRange3topFE"(%"7runtime5UInt811ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5UInt811ClosedRangeS" %0, 1
  ret i8 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime5UInt85Range4initF7runtime5UInt8S7runtime5UInt8SE4from2to"(%"7runtime5UInt85RangeS"* nocapture writeonly %0, i8 %1, i8 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime5UInt85RangeS", %"7runtime5UInt85RangeS"* %0, i64 0, i32 0
  store i8 %1, i8* %gep, align 1
  %gep3 = getelementptr inbounds %"7runtime5UInt85RangeS", %"7runtime5UInt85RangeS"* %0, i64 0, i32 1
  store i8 %2, i8* %gep3, align 1
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime5UInt85Range6bottomFE"(%"7runtime5UInt85RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime5UInt85RangeS" %0, 0
  ret i8 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime5UInt85Range3topFE"(%"7runtime5UInt85RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime5UInt85RangeS" %0, 1
  ret i8 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i8 @"7runtime5UInt88Optional6unwrapF7runtime5UInt8SE6orElse"(%"7runtime5UInt88OptionalV" %0, i8 %1) local_unnamed_addr #0 {
bb:
  %.fca.0.extract = extractvalue %"7runtime5UInt88OptionalV" %0, 0
  %.fca.2.0.extract = extractvalue %"7runtime5UInt88OptionalV" %0, 2, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  %_2.0 = select i1 %cond, i8 %.fca.2.0.extract, i8 %1
  ret i8 %_2.0
}

define %"7runtime5UInt88OptionalV" @"7runtime5UInt88Optional3mapFF7runtime5UInt8SE7runtime5UInt8SE0"(%"7runtime5UInt88OptionalV" %0, i8 (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime5UInt88OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.0.extract = extractvalue %"7runtime5UInt88OptionalV" %0, 2, 0
  %call = tail call i8 %1(i8 %.fca.2.0.extract)
  %copy8.fca.2.0.insert = insertvalue %"7runtime5UInt88OptionalV" { i32 1, {} poison, [1 x i8] poison }, i8 %call, 2, 0
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5UInt88OptionalV" [ %copy8.fca.2.0.insert, %bb3 ], [ { i32 0, {} poison, [1 x i8] undef }, %bb ]
  ret %"7runtime5UInt88OptionalV" %_2.0
}

define %"7runtime5UInt88OptionalV" @"7runtime5UInt88Optional7flatMapFF7runtime5UInt8SE7runtime5UInt88OptionalVE0"(%"7runtime5UInt88OptionalV" %0, %"7runtime5UInt88OptionalV" (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime5UInt88OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.0.extract = extractvalue %"7runtime5UInt88OptionalV" %0, 2, 0
  %call = tail call %"7runtime5UInt88OptionalV" %1(i8 %.fca.2.0.extract)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime5UInt88OptionalV" [ %call, %bb3 ], [ { i32 0, {} poison, [1 x i8] undef }, %bb ]
  ret %"7runtime5UInt88OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt163addF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %iadd = add i16 %1, %0
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %iadd)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt163subF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %isub = sub i16 %0, %1
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %isub)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt163mulF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %imul = mul i16 %1, %0
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %imul)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt163divF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i16, align 2
  %_4 = alloca i16, align 2
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_3, i16 0)
  %copy1 = load i16, i16* %_3, align 2
  %call = tail call i1 @"7runtime6UInt165equalF7runtime6UInt16S7runtime6UInt16SE00"(i16 %1, i16 %copy1)
  %idiv = udiv i16 %0, %1
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_4, i16 %idiv)
  %copy6 = load i16, i16* %_4, align 2
  ret i16 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt163modF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i16, align 2
  %_4 = alloca i16, align 2
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_3, i16 0)
  %copy1 = load i16, i16* %_3, align 2
  %call = tail call i1 @"7runtime6UInt165equalF7runtime6UInt16S7runtime6UInt16SE00"(i16 %1, i16 %copy1)
  %irem = urem i16 %0, %1
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_4, i16 %irem)
  %copy6 = load i16, i16* %_4, align 2
  ret i16 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt165bitOrF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ior = or i16 %1, %0
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %ior)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt166bitXorF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ixor = xor i16 %1, %0
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %ixor)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt166bitAndF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %iand = and i16 %1, %0
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %iand)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt169shiftLeftF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ishl = shl i16 %0, %1
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %ishl)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt1610shiftRightF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i16, align 2
  %ishr = lshr i16 %0, %1
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_2, i16 %ishr)
  %copy3 = load i16, i16* %_2, align 2
  ret i16 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt165equalF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt168notEqualF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt168lessThanF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp ult i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt1611greaterThanF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp ugt i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt1610lessThanEqF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp ule i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt1613greaterThanEqF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp uge i16 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime6UInt164unitF7runtime6UInt16SE0"(i16 returned %0) local_unnamed_addr #0 {
bb:
  ret i16 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime6UInt165RangeS" @"7runtime6UInt169openRangeF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime6UInt165RangeS", align 8
  call void @"7runtime6UInt165Range4initF7runtime6UInt16S7runtime6UInt16SE4from2to"(%"7runtime6UInt165RangeS"* nonnull %_2, i16 %0, i16 %1)
  %copy2.elt = getelementptr inbounds %"7runtime6UInt165RangeS", %"7runtime6UInt165RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i16, i16* %copy2.elt, align 8
  %2 = insertvalue %"7runtime6UInt165RangeS" undef, i16 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime6UInt165RangeS", %"7runtime6UInt165RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i16, i16* %copy2.elt3, align 2
  %copy25 = insertvalue %"7runtime6UInt165RangeS" %2, i16 %copy2.unpack4, 1
  ret %"7runtime6UInt165RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime6UInt1611ClosedRangeS" @"7runtime6UInt1611closedRangeF7runtime6UInt16S7runtime6UInt16SE00"(i16 %0, i16 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime6UInt1611ClosedRangeS", align 8
  call void @"7runtime6UInt1611ClosedRange4initF7runtime6UInt16S7runtime6UInt16SE4from2to"(%"7runtime6UInt1611ClosedRangeS"* nonnull %_2, i16 %0, i16 %1)
  %copy2.elt = getelementptr inbounds %"7runtime6UInt1611ClosedRangeS", %"7runtime6UInt1611ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i16, i16* %copy2.elt, align 8
  %2 = insertvalue %"7runtime6UInt1611ClosedRangeS" undef, i16 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime6UInt1611ClosedRangeS", %"7runtime6UInt1611ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i16, i16* %copy2.elt3, align 2
  %copy25 = insertvalue %"7runtime6UInt1611ClosedRangeS" %2, i16 %copy2.unpack4, 1
  ret %"7runtime6UInt1611ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt166negateF7runtime6UInt16SE0"(i16 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i16, align 2
  %ineg = sub i16 0, %0
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_1, i16 %ineg)
  %copy2 = load i16, i16* %_1, align 2
  ret i16 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i16 @"7runtime6UInt166invertF7runtime6UInt16SE0"(i16 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i16, align 2
  %iinv = xor i16 %0, -1
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_1, i16 %iinv)
  %copy2 = load i16, i16* %_1, align 2
  ret i16 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt164initF7runtime5UInt8SE0"(i16* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext16 = zext i8 %1 to i16
  store i16 %izext16, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt164initF7runtime6UInt32SE10truncating"(i16* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %itrunc16 = trunc i32 %1 to i16
  store i16 %itrunc16, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt164initF7runtime6UInt64SE10truncating"(i16* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %itrunc16 = trunc i64 %1 to i16
  store i16 %itrunc16, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt164initF7runtime5Int16SE7bitcast"(i16* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  store i16 %1, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt164initFlE4repr"(i16* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  store i16 %1, i16* %0, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt1611ClosedRange4initF7runtime6UInt16S7runtime6UInt16SE4from2to"(%"7runtime6UInt1611ClosedRangeS"* nocapture writeonly %0, i16 %1, i16 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime6UInt1611ClosedRangeS", %"7runtime6UInt1611ClosedRangeS"* %0, i64 0, i32 0
  store i16 %1, i16* %gep, align 2
  %gep3 = getelementptr inbounds %"7runtime6UInt1611ClosedRangeS", %"7runtime6UInt1611ClosedRangeS"* %0, i64 0, i32 1
  store i16 %2, i16* %gep3, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime6UInt1611ClosedRange6bottomFE"(%"7runtime6UInt1611ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt1611ClosedRangeS" %0, 0
  ret i16 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime6UInt1611ClosedRange3topFE"(%"7runtime6UInt1611ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime6UInt1611ClosedRangeS" %0, 1
  ret i16 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt165Range4initF7runtime6UInt16S7runtime6UInt16SE4from2to"(%"7runtime6UInt165RangeS"* nocapture writeonly %0, i16 %1, i16 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime6UInt165RangeS", %"7runtime6UInt165RangeS"* %0, i64 0, i32 0
  store i16 %1, i16* %gep, align 2
  %gep3 = getelementptr inbounds %"7runtime6UInt165RangeS", %"7runtime6UInt165RangeS"* %0, i64 0, i32 1
  store i16 %2, i16* %gep3, align 2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime6UInt165Range6bottomFE"(%"7runtime6UInt165RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt165RangeS" %0, 0
  ret i16 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime6UInt165Range3topFE"(%"7runtime6UInt165RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime6UInt165RangeS" %0, 1
  ret i16 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i16 @"7runtime6UInt168Optional6unwrapF7runtime6UInt16SE6orElse"(%"7runtime6UInt168OptionalV" %0, i16 %1) local_unnamed_addr #0 {
bb:
  %.fca.0.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 0
  %.fca.2.0.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 2, 0
  %.fca.2.1.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 2, 1
  %cond = icmp eq i32 %.fca.0.extract, 1
  %_0.sroa.4.4.insert.ext = zext i8 %.fca.2.1.extract to i16
  %_0.sroa.4.4.insert.shift = shl nuw i16 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.2.4.insert.ext = zext i8 %.fca.2.0.extract to i16
  %_0.sroa.2.4.insert.insert = or i16 %_0.sroa.4.4.insert.shift, %_0.sroa.2.4.insert.ext
  %_2.0 = select i1 %cond, i16 %_0.sroa.2.4.insert.insert, i16 %1
  ret i16 %_2.0
}

define %"7runtime6UInt168OptionalV" @"7runtime6UInt168Optional3mapFF7runtime6UInt16SE7runtime6UInt16SE0"(%"7runtime6UInt168OptionalV" %0, i16 (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.1.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 2, 1
  %.fca.2.0.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 2, 0
  %_0.sroa.4.4.insert.ext = zext i8 %.fca.2.1.extract to i16
  %_0.sroa.4.4.insert.shift = shl nuw i16 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.2.4.insert.ext = zext i8 %.fca.2.0.extract to i16
  %_0.sroa.2.4.insert.insert = or i16 %_0.sroa.4.4.insert.shift, %_0.sroa.2.4.insert.ext
  %call = tail call i16 %1(i16 %_0.sroa.2.4.insert.insert)
  %_3.sroa.2.4.extract.trunc = trunc i16 %call to i8
  %_3.sroa.4.4.extract.shift = lshr i16 %call, 8
  %_3.sroa.4.4.extract.trunc = trunc i16 %_3.sroa.4.4.extract.shift to i8
  %copy8.fca.2.0.insert = insertvalue %"7runtime6UInt168OptionalV" { i32 1, {} poison, [2 x i8] poison }, i8 %_3.sroa.2.4.extract.trunc, 2, 0
  %copy8.fca.2.1.insert = insertvalue %"7runtime6UInt168OptionalV" %copy8.fca.2.0.insert, i8 %_3.sroa.4.4.extract.trunc, 2, 1
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6UInt168OptionalV" [ %copy8.fca.2.1.insert, %bb3 ], [ { i32 0, {} poison, [2 x i8] undef }, %bb ]
  ret %"7runtime6UInt168OptionalV" %_2.0
}

define %"7runtime6UInt168OptionalV" @"7runtime6UInt168Optional7flatMapFF7runtime6UInt16SE7runtime6UInt168OptionalVE0"(%"7runtime6UInt168OptionalV" %0, %"7runtime6UInt168OptionalV" (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.fca.0.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 0
  %cond = icmp eq i32 %.fca.0.extract, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.fca.2.1.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 2, 1
  %.fca.2.0.extract = extractvalue %"7runtime6UInt168OptionalV" %0, 2, 0
  %_0.sroa.4.4.insert.ext = zext i8 %.fca.2.1.extract to i16
  %_0.sroa.4.4.insert.shift = shl nuw i16 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.2.4.insert.ext = zext i8 %.fca.2.0.extract to i16
  %_0.sroa.2.4.insert.insert = or i16 %_0.sroa.4.4.insert.shift, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime6UInt168OptionalV" %1(i16 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6UInt168OptionalV" [ %call, %bb3 ], [ { i32 0, {} poison, [2 x i8] undef }, %bb ]
  ret %"7runtime6UInt168OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt323addF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %iadd = add i32 %1, %0
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %iadd)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt323subF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %isub = sub i32 %0, %1
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %isub)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt323mulF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %imul = mul i32 %1, %0
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %imul)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt323divF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_4 = alloca i32, align 4
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_3, i32 0)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime6UInt325equalF7runtime6UInt32S7runtime6UInt32SE00"(i32 %1, i32 %copy1)
  %idiv = udiv i32 %0, %1
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_4, i32 %idiv)
  %copy6 = load i32, i32* %_4, align 4
  ret i32 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt323modF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_4 = alloca i32, align 4
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_3, i32 0)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime6UInt325equalF7runtime6UInt32S7runtime6UInt32SE00"(i32 %1, i32 %copy1)
  %irem = urem i32 %0, %1
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_4, i32 %irem)
  %copy6 = load i32, i32* %_4, align 4
  ret i32 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt325bitOrF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ior = or i32 %1, %0
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %ior)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt326bitXorF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ixor = xor i32 %1, %0
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %ixor)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt326bitAndF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %iand = and i32 %1, %0
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %iand)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt329shiftLeftF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ishl = shl i32 %0, %1
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %ishl)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt3210shiftRightF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i32, align 4
  %ishr = lshr i32 %0, %1
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_2, i32 %ishr)
  %copy3 = load i32, i32* %_2, align 4
  ret i32 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt325equalF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt328notEqualF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt328lessThanF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp ult i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt3211greaterThanF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp ugt i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt3210lessThanEqF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp ule i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt3213greaterThanEqF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp uge i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime6UInt324unitF7runtime6UInt32SE0"(i32 returned %0) local_unnamed_addr #0 {
bb:
  ret i32 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime6UInt325RangeS" @"7runtime6UInt329openRangeF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime6UInt325RangeS", align 8
  call void @"7runtime6UInt325Range4initF7runtime6UInt32S7runtime6UInt32SE4from2to"(%"7runtime6UInt325RangeS"* nonnull %_2, i32 %0, i32 %1)
  %copy2.elt = getelementptr inbounds %"7runtime6UInt325RangeS", %"7runtime6UInt325RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i32, i32* %copy2.elt, align 8
  %2 = insertvalue %"7runtime6UInt325RangeS" undef, i32 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime6UInt325RangeS", %"7runtime6UInt325RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i32, i32* %copy2.elt3, align 4
  %copy25 = insertvalue %"7runtime6UInt325RangeS" %2, i32 %copy2.unpack4, 1
  ret %"7runtime6UInt325RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime6UInt3211ClosedRangeS" @"7runtime6UInt3211closedRangeF7runtime6UInt32S7runtime6UInt32SE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime6UInt3211ClosedRangeS", align 8
  call void @"7runtime6UInt3211ClosedRange4initF7runtime6UInt32S7runtime6UInt32SE4from2to"(%"7runtime6UInt3211ClosedRangeS"* nonnull %_2, i32 %0, i32 %1)
  %copy2.elt = getelementptr inbounds %"7runtime6UInt3211ClosedRangeS", %"7runtime6UInt3211ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i32, i32* %copy2.elt, align 8
  %2 = insertvalue %"7runtime6UInt3211ClosedRangeS" undef, i32 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime6UInt3211ClosedRangeS", %"7runtime6UInt3211ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i32, i32* %copy2.elt3, align 4
  %copy25 = insertvalue %"7runtime6UInt3211ClosedRangeS" %2, i32 %copy2.unpack4, 1
  ret %"7runtime6UInt3211ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt326negateF7runtime6UInt32SE0"(i32 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i32, align 4
  %ineg = sub i32 0, %0
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_1, i32 %ineg)
  %copy2 = load i32, i32* %_1, align 4
  ret i32 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i32 @"7runtime6UInt326invertF7runtime6UInt32SE0"(i32 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i32, align 4
  %iinv = xor i32 %0, -1
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_1, i32 %iinv)
  %copy2 = load i32, i32* %_1, align 4
  ret i32 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt324initF7runtime5UInt8SE0"(i32* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext32 = zext i8 %1 to i32
  store i32 %izext32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt324initF7runtime6UInt16SE0"(i32* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext32 = zext i16 %1 to i32
  store i32 %izext32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt324initF7runtime6UInt64SE10truncating"(i32* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  %itrunc32 = trunc i64 %1 to i32
  store i32 %itrunc32, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt324initF7runtime5Int32SE7bitcast"(i32* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  store i32 %1, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt324initFjE4repr"(i32* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  store i32 %1, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt3211ClosedRange4initF7runtime6UInt32S7runtime6UInt32SE4from2to"(%"7runtime6UInt3211ClosedRangeS"* nocapture writeonly %0, i32 %1, i32 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime6UInt3211ClosedRangeS", %"7runtime6UInt3211ClosedRangeS"* %0, i64 0, i32 0
  store i32 %1, i32* %gep, align 4
  %gep3 = getelementptr inbounds %"7runtime6UInt3211ClosedRangeS", %"7runtime6UInt3211ClosedRangeS"* %0, i64 0, i32 1
  store i32 %2, i32* %gep3, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime6UInt3211ClosedRange6bottomFE"(%"7runtime6UInt3211ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt3211ClosedRangeS" %0, 0
  ret i32 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime6UInt3211ClosedRange3topFE"(%"7runtime6UInt3211ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime6UInt3211ClosedRangeS" %0, 1
  ret i32 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt325Range4initF7runtime6UInt32S7runtime6UInt32SE4from2to"(%"7runtime6UInt325RangeS"* nocapture writeonly %0, i32 %1, i32 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime6UInt325RangeS", %"7runtime6UInt325RangeS"* %0, i64 0, i32 0
  store i32 %1, i32* %gep, align 4
  %gep3 = getelementptr inbounds %"7runtime6UInt325RangeS", %"7runtime6UInt325RangeS"* %0, i64 0, i32 1
  store i32 %2, i32* %gep3, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime6UInt325Range6bottomFE"(%"7runtime6UInt325RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt325RangeS" %0, 0
  ret i32 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime6UInt325Range3topFE"(%"7runtime6UInt325RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime6UInt325RangeS" %0, 1
  ret i32 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime6UInt328Optional6unwrapF7runtime6UInt32SE6orElse"(%"7runtime6UInt328OptionalV" %0, i32 %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt328OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime6UInt328OptionalV" %0, 2
  %.elt7.elt = extractvalue [4 x i8] %.elt7, 0
  %.elt7.elt9 = extractvalue [4 x i8] %.elt7, 1
  %.elt7.elt11 = extractvalue [4 x i8] %.elt7, 2
  %.elt7.elt13 = extractvalue [4 x i8] %.elt7, 3
  %cond = icmp eq i32 %.elt, 1
  %_0.sroa.6.4.insert.ext = zext i8 %.elt7.elt13 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt7.elt11 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt7.elt9 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %_2.0 = select i1 %cond, i32 %_0.sroa.2.4.insert.insert, i32 %1
  ret i32 %_2.0
}

define %"7runtime6UInt328OptionalV" @"7runtime6UInt328Optional3mapFF7runtime6UInt32SE7runtime6UInt32SE0"(%"7runtime6UInt328OptionalV" %0, i32 (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6UInt328OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime6UInt328OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt21 = extractvalue [4 x i8] %.elt15, 3
  %.elt15.elt19 = extractvalue [4 x i8] %.elt15, 2
  %.elt15.elt17 = extractvalue [4 x i8] %.elt15, 1
  %.elt15.elt = extractvalue [4 x i8] %.elt15, 0
  %_0.sroa.6.4.insert.ext = zext i8 %.elt15.elt21 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt15.elt19 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt15.elt17 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %call = tail call i32 %1(i32 %_0.sroa.2.4.insert.insert)
  %_3.sroa.1.4.extract.trunc = trunc i32 %call to i8
  %_3.sroa.3.4.extract.shift = lshr i32 %call, 8
  %_3.sroa.3.4.extract.trunc = trunc i32 %_3.sroa.3.4.extract.shift to i8
  %_3.sroa.4.4.extract.shift = lshr i32 %call, 16
  %_3.sroa.4.4.extract.trunc = trunc i32 %_3.sroa.4.4.extract.shift to i8
  %_3.sroa.5.4.extract.shift = lshr i32 %call, 24
  %_3.sroa.5.4.extract.trunc = trunc i32 %_3.sroa.5.4.extract.shift to i8
  %2 = insertvalue [4 x i8] undef, i8 %_3.sroa.1.4.extract.trunc, 0
  %3 = insertvalue [4 x i8] %2, i8 %_3.sroa.3.4.extract.trunc, 1
  %4 = insertvalue [4 x i8] %3, i8 %_3.sroa.4.4.extract.trunc, 2
  %copy8.unpack3745 = insertvalue [4 x i8] %4, i8 %_3.sroa.5.4.extract.trunc, 3
  %copy838 = insertvalue %"7runtime6UInt328OptionalV" { i32 1, {} undef, [4 x i8] undef }, [4 x i8] %copy8.unpack3745, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6UInt328OptionalV" [ %copy838, %bb3 ], [ { i32 0, {} undef, [4 x i8] undef }, %bb ]
  ret %"7runtime6UInt328OptionalV" %_2.0
}

define %"7runtime6UInt328OptionalV" @"7runtime6UInt328Optional7flatMapFF7runtime6UInt32SE7runtime6UInt328OptionalVE0"(%"7runtime6UInt328OptionalV" %0, %"7runtime6UInt328OptionalV" (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6UInt328OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime6UInt328OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt16 = extractvalue [4 x i8] %.elt10, 3
  %.elt10.elt14 = extractvalue [4 x i8] %.elt10, 2
  %.elt10.elt12 = extractvalue [4 x i8] %.elt10, 1
  %.elt10.elt = extractvalue [4 x i8] %.elt10, 0
  %_0.sroa.6.4.insert.ext = zext i8 %.elt10.elt16 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt10.elt14 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt10.elt12 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime6UInt328OptionalV" %1(i32 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6UInt328OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [4 x i8] undef }, %bb ]
  ret %"7runtime6UInt328OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt643addF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iadd = add i64 %1, %0
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %iadd)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt643subF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %isub = sub i64 %0, %1
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %isub)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt643mulF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %imul = mul i64 %1, %0
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %imul)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt643divF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime6UInt645equalF7runtime6UInt64S7runtime6UInt64SE00"(i64 %1, i64 %copy1)
  %idiv = udiv i64 %0, %1
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_4, i64 %idiv)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt643modF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_3 = alloca i64, align 8
  %_4 = alloca i64, align 8
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_3, i64 0)
  %copy1 = load i64, i64* %_3, align 8
  %call = tail call i1 @"7runtime6UInt645equalF7runtime6UInt64S7runtime6UInt64SE00"(i64 %1, i64 %copy1)
  %irem = urem i64 %0, %1
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_4, i64 %irem)
  %copy6 = load i64, i64* %_4, align 8
  ret i64 %copy6
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt645bitOrF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ior = or i64 %1, %0
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %ior)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt646bitXorF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ixor = xor i64 %1, %0
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %ixor)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt646bitAndF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %iand = and i64 %1, %0
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %iand)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt649shiftLeftF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishl = shl i64 %0, %1
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %ishl)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt6410shiftRightF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i64, align 8
  %ishr = lshr i64 %0, %1
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_2, i64 %ishr)
  %copy3 = load i64, i64* %_2, align 8
  ret i64 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt645equalF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt648notEqualF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt648lessThanF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp ult i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt6411greaterThanF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp ugt i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt6410lessThanEqF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp ule i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime6UInt6413greaterThanEqF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp uge i64 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime6UInt644unitF7runtime6UInt64SE0"(i64 returned %0) local_unnamed_addr #0 {
bb:
  ret i64 %0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime6UInt645RangeS" @"7runtime6UInt649openRangeF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime6UInt645RangeS", align 8
  call void @"7runtime6UInt645Range4initF7runtime6UInt64S7runtime6UInt64SE4from2to"(%"7runtime6UInt645RangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime6UInt645RangeS", %"7runtime6UInt645RangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime6UInt645RangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime6UInt645RangeS", %"7runtime6UInt645RangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime6UInt645RangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime6UInt645RangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define %"7runtime6UInt6411ClosedRangeS" @"7runtime6UInt6411closedRangeF7runtime6UInt64S7runtime6UInt64SE00"(i64 %0, i64 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca %"7runtime6UInt6411ClosedRangeS", align 8
  call void @"7runtime6UInt6411ClosedRange4initF7runtime6UInt64S7runtime6UInt64SE4from2to"(%"7runtime6UInt6411ClosedRangeS"* nonnull %_2, i64 %0, i64 %1)
  %copy2.elt = getelementptr inbounds %"7runtime6UInt6411ClosedRangeS", %"7runtime6UInt6411ClosedRangeS"* %_2, i64 0, i32 0
  %copy2.unpack = load i64, i64* %copy2.elt, align 8
  %2 = insertvalue %"7runtime6UInt6411ClosedRangeS" undef, i64 %copy2.unpack, 0
  %copy2.elt3 = getelementptr inbounds %"7runtime6UInt6411ClosedRangeS", %"7runtime6UInt6411ClosedRangeS"* %_2, i64 0, i32 1
  %copy2.unpack4 = load i64, i64* %copy2.elt3, align 8
  %copy25 = insertvalue %"7runtime6UInt6411ClosedRangeS" %2, i64 %copy2.unpack4, 1
  ret %"7runtime6UInt6411ClosedRangeS" %copy25
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt646negateF7runtime6UInt64SE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %ineg = sub i64 0, %0
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_1, i64 %ineg)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6UInt646invertF7runtime6UInt64SE0"(i64 %0) local_unnamed_addr #2 {
bb:
  %_1 = alloca i64, align 8
  %iinv = xor i64 %0, -1
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_1, i64 %iinv)
  %copy2 = load i64, i64* %_1, align 8
  ret i64 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime5FloatSE5floor"(i64* nocapture writeonly %0, float %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi float %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime6DoubleSE5floor"(i64* nocapture writeonly %0, double %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi double %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime4HalfSE5floor"(i64* nocapture writeonly %0, half %1) local_unnamed_addr #2 {
bb:
  %fcnvi = fptosi half %1 to i64
  store i64 %fcnvi, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime5UInt8SE0"(i64* nocapture writeonly %0, i8 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i8 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime6UInt16SE0"(i64* nocapture writeonly %0, i16 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i16 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime6UInt32SE0"(i64* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  %izext64 = zext i32 %1 to i64
  store i64 %izext64, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime5Int64SE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime3IntSE7bitcast"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initF7runtime4UIntSE0"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt644initFiE4repr"(i64* nocapture writeonly %0, i64 %1) local_unnamed_addr #2 {
bb:
  store i64 %1, i64* %0, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt6411ClosedRange4initF7runtime6UInt64S7runtime6UInt64SE4from2to"(%"7runtime6UInt6411ClosedRangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime6UInt6411ClosedRangeS", %"7runtime6UInt6411ClosedRangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime6UInt6411ClosedRangeS", %"7runtime6UInt6411ClosedRangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime6UInt6411ClosedRange6bottomFE"(%"7runtime6UInt6411ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt6411ClosedRangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime6UInt6411ClosedRange3topFE"(%"7runtime6UInt6411ClosedRangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime6UInt6411ClosedRangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6UInt645Range4initF7runtime6UInt64S7runtime6UInt64SE4from2to"(%"7runtime6UInt645RangeS"* nocapture writeonly %0, i64 %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime6UInt645RangeS", %"7runtime6UInt645RangeS"* %0, i64 0, i32 0
  store i64 %1, i64* %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime6UInt645RangeS", %"7runtime6UInt645RangeS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime6UInt645Range6bottomFE"(%"7runtime6UInt645RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt645RangeS" %0, 0
  ret i64 %.elt
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime6UInt645Range3topFE"(%"7runtime6UInt645RangeS" %0) local_unnamed_addr #0 {
bb:
  %.elt2 = extractvalue %"7runtime6UInt645RangeS" %0, 1
  ret i64 %.elt2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @"7runtime6UInt648Optional6unwrapF7runtime6UInt64SE6orElse"(%"7runtime6UInt648OptionalV" %0, i64 %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6UInt648OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime6UInt648OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt7.elt21 = extractvalue [8 x i8] %.elt7, 7
  %.elt7.elt19 = extractvalue [8 x i8] %.elt7, 6
  %.elt7.elt17 = extractvalue [8 x i8] %.elt7, 5
  %.elt7.elt15 = extractvalue [8 x i8] %.elt7, 4
  %.elt7.elt13 = extractvalue [8 x i8] %.elt7, 3
  %.elt7.elt11 = extractvalue [8 x i8] %.elt7, 2
  %.elt7.elt9 = extractvalue [8 x i8] %.elt7, 1
  %.elt7.elt = extractvalue [8 x i8] %.elt7, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt7.elt21 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt7.elt19 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt7.elt17 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt7.elt15 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt7.elt13 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt7.elt11 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt7.elt9 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi i64 [ %_0.sroa.2.4.insert.insert, %bb3 ], [ %1, %bb ]
  ret i64 %_2.0
}

define %"7runtime6UInt648OptionalV" @"7runtime6UInt648Optional3mapFF7runtime6UInt64SE7runtime6UInt64SE0"(%"7runtime6UInt648OptionalV" %0, i64 (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6UInt648OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime6UInt648OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt29 = extractvalue [8 x i8] %.elt15, 7
  %.elt15.elt27 = extractvalue [8 x i8] %.elt15, 6
  %.elt15.elt25 = extractvalue [8 x i8] %.elt15, 5
  %.elt15.elt23 = extractvalue [8 x i8] %.elt15, 4
  %.elt15.elt21 = extractvalue [8 x i8] %.elt15, 3
  %.elt15.elt19 = extractvalue [8 x i8] %.elt15, 2
  %.elt15.elt17 = extractvalue [8 x i8] %.elt15, 1
  %.elt15.elt = extractvalue [8 x i8] %.elt15, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt15.elt29 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt15.elt27 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt15.elt25 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt15.elt23 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt15.elt21 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt15.elt19 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt15.elt17 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call i64 %1(i64 %_0.sroa.2.4.insert.insert)
  %_3.sroa.1.4.extract.trunc = trunc i64 %call to i8
  %_3.sroa.3.4.extract.shift = lshr i64 %call, 8
  %_3.sroa.3.4.extract.trunc = trunc i64 %_3.sroa.3.4.extract.shift to i8
  %_3.sroa.4.4.extract.shift = lshr i64 %call, 16
  %_3.sroa.4.4.extract.trunc = trunc i64 %_3.sroa.4.4.extract.shift to i8
  %_3.sroa.5.4.extract.shift = lshr i64 %call, 24
  %_3.sroa.5.4.extract.trunc = trunc i64 %_3.sroa.5.4.extract.shift to i8
  %_3.sroa.6.4.extract.shift = lshr i64 %call, 32
  %_3.sroa.6.4.extract.trunc = trunc i64 %_3.sroa.6.4.extract.shift to i8
  %_3.sroa.7.4.extract.shift = lshr i64 %call, 40
  %_3.sroa.7.4.extract.trunc = trunc i64 %_3.sroa.7.4.extract.shift to i8
  %_3.sroa.8.4.extract.shift = lshr i64 %call, 48
  %_3.sroa.8.4.extract.trunc = trunc i64 %_3.sroa.8.4.extract.shift to i8
  %_3.sroa.9.4.extract.shift = lshr i64 %call, 56
  %_3.sroa.9.4.extract.trunc = trunc i64 %_3.sroa.9.4.extract.shift to i8
  %2 = insertvalue [8 x i8] undef, i8 %_3.sroa.1.4.extract.trunc, 0
  %3 = insertvalue [8 x i8] %2, i8 %_3.sroa.3.4.extract.trunc, 1
  %4 = insertvalue [8 x i8] %3, i8 %_3.sroa.4.4.extract.trunc, 2
  %5 = insertvalue [8 x i8] %4, i8 %_3.sroa.5.4.extract.trunc, 3
  %6 = insertvalue [8 x i8] %5, i8 %_3.sroa.6.4.extract.trunc, 4
  %7 = insertvalue [8 x i8] %6, i8 %_3.sroa.7.4.extract.trunc, 5
  %8 = insertvalue [8 x i8] %7, i8 %_3.sroa.8.4.extract.trunc, 6
  %copy8.unpack5369 = insertvalue [8 x i8] %8, i8 %_3.sroa.9.4.extract.trunc, 7
  %copy854 = insertvalue %"7runtime6UInt648OptionalV" { i32 1, {} undef, [8 x i8] undef }, [8 x i8] %copy8.unpack5369, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6UInt648OptionalV" [ %copy854, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime6UInt648OptionalV" %_2.0
}

define %"7runtime6UInt648OptionalV" @"7runtime6UInt648Optional7flatMapFF7runtime6UInt64SE7runtime6UInt648OptionalVE0"(%"7runtime6UInt648OptionalV" %0, %"7runtime6UInt648OptionalV" (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6UInt648OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime6UInt648OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt24 = extractvalue [8 x i8] %.elt10, 7
  %.elt10.elt22 = extractvalue [8 x i8] %.elt10, 6
  %.elt10.elt20 = extractvalue [8 x i8] %.elt10, 5
  %.elt10.elt18 = extractvalue [8 x i8] %.elt10, 4
  %.elt10.elt16 = extractvalue [8 x i8] %.elt10, 3
  %.elt10.elt14 = extractvalue [8 x i8] %.elt10, 2
  %.elt10.elt12 = extractvalue [8 x i8] %.elt10, 1
  %.elt10.elt = extractvalue [8 x i8] %.elt10, 0
  %_0.sroa.10.4.insert.ext = zext i8 %.elt10.elt24 to i64
  %_0.sroa.10.4.insert.shift = shl nuw i64 %_0.sroa.10.4.insert.ext, 56
  %_0.sroa.9.4.insert.ext = zext i8 %.elt10.elt22 to i64
  %_0.sroa.9.4.insert.shift = shl nuw nsw i64 %_0.sroa.9.4.insert.ext, 48
  %_0.sroa.9.4.insert.insert = or i64 %_0.sroa.10.4.insert.shift, %_0.sroa.9.4.insert.shift
  %_0.sroa.8.4.insert.ext = zext i8 %.elt10.elt20 to i64
  %_0.sroa.8.4.insert.shift = shl nuw nsw i64 %_0.sroa.8.4.insert.ext, 40
  %_0.sroa.8.4.insert.insert = or i64 %_0.sroa.9.4.insert.insert, %_0.sroa.8.4.insert.shift
  %_0.sroa.7.4.insert.ext = zext i8 %.elt10.elt18 to i64
  %_0.sroa.7.4.insert.shift = shl nuw nsw i64 %_0.sroa.7.4.insert.ext, 32
  %_0.sroa.7.4.insert.insert = or i64 %_0.sroa.8.4.insert.insert, %_0.sroa.7.4.insert.shift
  %_0.sroa.6.4.insert.ext = zext i8 %.elt10.elt16 to i64
  %_0.sroa.6.4.insert.shift = shl nuw nsw i64 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.6.4.insert.insert = or i64 %_0.sroa.7.4.insert.insert, %_0.sroa.6.4.insert.shift
  %_0.sroa.5.4.insert.ext = zext i8 %.elt10.elt14 to i64
  %_0.sroa.5.4.insert.shift = shl nuw nsw i64 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.4.4.insert.ext = zext i8 %.elt10.elt12 to i64
  %_0.sroa.4.4.insert.shift = shl nuw nsw i64 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.mask = or i64 %_0.sroa.6.4.insert.insert, %_0.sroa.5.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i64
  %_0.sroa.2.4.insert.mask = or i64 %_0.sroa.4.4.insert.mask, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.insert = or i64 %_0.sroa.2.4.insert.mask, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime6UInt648OptionalV" %1(i64 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6UInt648OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [8 x i8] undef }, %bb ]
  ret %"7runtime6UInt648OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i64 @"7runtime6String6lengthFE"(%"7runtime6StringS" %0) local_unnamed_addr #2 {
bb:
  %.elt3 = extractvalue %"7runtime6StringS" %0, 1
  %_1 = alloca i64, align 8
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_1, i64 %.elt3)
  %copy1 = load i64, i64* %_1, align 8
  ret i64 %copy1
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime6String4initFtiE3ptr3len"(%"7runtime6StringS"* nocapture writeonly %0, i8* %1, i64 %2) local_unnamed_addr #2 {
bb:
  %gep = getelementptr inbounds %"7runtime6StringS", %"7runtime6StringS"* %0, i64 0, i32 0
  store i8* %1, i8** %gep, align 8
  %gep3 = getelementptr inbounds %"7runtime6StringS", %"7runtime6StringS"* %0, i64 0, i32 1
  store i64 %2, i64* %gep3, align 8
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define %"7runtime6StringS" @"7runtime6String8Optional6unwrapF7runtime6StringSE6orElse"(%"7runtime6String8OptionalV" %0, %"7runtime6StringS" %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime6String8OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime6String8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt7.elt37 = extractvalue [16 x i8] %.elt7, 15
  %.elt7.elt35 = extractvalue [16 x i8] %.elt7, 14
  %.elt7.elt33 = extractvalue [16 x i8] %.elt7, 13
  %.elt7.elt31 = extractvalue [16 x i8] %.elt7, 12
  %.elt7.elt29 = extractvalue [16 x i8] %.elt7, 11
  %.elt7.elt27 = extractvalue [16 x i8] %.elt7, 10
  %.elt7.elt25 = extractvalue [16 x i8] %.elt7, 9
  %.elt7.elt23 = extractvalue [16 x i8] %.elt7, 8
  %.elt7.elt9 = extractvalue [16 x i8] %.elt7, 1
  %_0.sroa.2.5.insert.ext = zext i8 %.elt7.elt9 to i64
  %_0.sroa.2.5.insert.shift = shl nuw nsw i64 %_0.sroa.2.5.insert.ext, 8
  %.elt7.elt = extractvalue [16 x i8] %.elt7, 0
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i64
  %_0.sroa.2.5.insert.insert = or i64 %_0.sroa.2.5.insert.shift, %_0.sroa.2.4.insert.ext
  %.elt7.elt11 = extractvalue [16 x i8] %.elt7, 2
  %_0.sroa.2.6.insert.ext = zext i8 %.elt7.elt11 to i64
  %_0.sroa.2.6.insert.shift = shl nuw nsw i64 %_0.sroa.2.6.insert.ext, 16
  %_0.sroa.2.6.insert.insert = or i64 %_0.sroa.2.5.insert.insert, %_0.sroa.2.6.insert.shift
  %.elt7.elt13 = extractvalue [16 x i8] %.elt7, 3
  %_0.sroa.2.7.insert.ext = zext i8 %.elt7.elt13 to i64
  %_0.sroa.2.7.insert.shift = shl nuw nsw i64 %_0.sroa.2.7.insert.ext, 24
  %_0.sroa.2.7.insert.insert = or i64 %_0.sroa.2.6.insert.insert, %_0.sroa.2.7.insert.shift
  %.elt7.elt15 = extractvalue [16 x i8] %.elt7, 4
  %_0.sroa.2.8.insert.ext = zext i8 %.elt7.elt15 to i64
  %_0.sroa.2.8.insert.shift = shl nuw nsw i64 %_0.sroa.2.8.insert.ext, 32
  %_0.sroa.2.9.insert.mask = or i64 %_0.sroa.2.7.insert.insert, %_0.sroa.2.8.insert.shift
  %.elt7.elt17 = extractvalue [16 x i8] %.elt7, 5
  %_0.sroa.2.9.insert.ext = zext i8 %.elt7.elt17 to i64
  %_0.sroa.2.9.insert.shift = shl nuw nsw i64 %_0.sroa.2.9.insert.ext, 40
  %_0.sroa.2.10.insert.mask.masked = or i64 %_0.sroa.2.9.insert.mask, %_0.sroa.2.9.insert.shift
  %.elt7.elt19 = extractvalue [16 x i8] %.elt7, 6
  %_0.sroa.2.10.insert.ext = zext i8 %.elt7.elt19 to i64
  %_0.sroa.2.10.insert.shift = shl nuw nsw i64 %_0.sroa.2.10.insert.ext, 48
  %.elt7.elt21 = extractvalue [16 x i8] %.elt7, 7
  %_0.sroa.2.11.insert.ext = zext i8 %.elt7.elt21 to i64
  %_0.sroa.2.11.insert.shift = shl nuw i64 %_0.sroa.2.11.insert.ext, 56
  %_0.sroa.2.11.insert.mask = or i64 %_0.sroa.2.11.insert.shift, %_0.sroa.2.10.insert.shift
  %_0.sroa.2.11.insert.insert = or i64 %_0.sroa.2.11.insert.mask, %_0.sroa.2.10.insert.mask.masked
  %2 = inttoptr i64 %_0.sroa.2.11.insert.insert to i8*
  %3 = insertvalue %"7runtime6StringS" undef, i8* %2, 0
  %_0.sroa.19.12.insert.ext = zext i8 %.elt7.elt37 to i64
  %_0.sroa.19.12.insert.shift = shl nuw i64 %_0.sroa.19.12.insert.ext, 56
  %_0.sroa.18.12.insert.ext = zext i8 %.elt7.elt35 to i64
  %_0.sroa.18.12.insert.shift = shl nuw nsw i64 %_0.sroa.18.12.insert.ext, 48
  %_0.sroa.18.12.insert.insert = or i64 %_0.sroa.19.12.insert.shift, %_0.sroa.18.12.insert.shift
  %_0.sroa.17.12.insert.ext = zext i8 %.elt7.elt33 to i64
  %_0.sroa.17.12.insert.shift = shl nuw nsw i64 %_0.sroa.17.12.insert.ext, 40
  %_0.sroa.17.12.insert.insert = or i64 %_0.sroa.18.12.insert.insert, %_0.sroa.17.12.insert.shift
  %_0.sroa.16.12.insert.ext = zext i8 %.elt7.elt31 to i64
  %_0.sroa.16.12.insert.shift = shl nuw nsw i64 %_0.sroa.16.12.insert.ext, 32
  %_0.sroa.16.12.insert.insert = or i64 %_0.sroa.17.12.insert.insert, %_0.sroa.16.12.insert.shift
  %_0.sroa.15.12.insert.ext = zext i8 %.elt7.elt29 to i64
  %_0.sroa.15.12.insert.shift = shl nuw nsw i64 %_0.sroa.15.12.insert.ext, 24
  %_0.sroa.15.12.insert.insert = or i64 %_0.sroa.16.12.insert.insert, %_0.sroa.15.12.insert.shift
  %_0.sroa.14.12.insert.ext = zext i8 %.elt7.elt27 to i64
  %_0.sroa.14.12.insert.shift = shl nuw nsw i64 %_0.sroa.14.12.insert.ext, 16
  %_0.sroa.13.12.insert.ext = zext i8 %.elt7.elt25 to i64
  %_0.sroa.13.12.insert.shift = shl nuw nsw i64 %_0.sroa.13.12.insert.ext, 8
  %_0.sroa.13.12.insert.mask = or i64 %_0.sroa.15.12.insert.insert, %_0.sroa.14.12.insert.shift
  %_0.sroa.11.12.insert.ext = zext i8 %.elt7.elt23 to i64
  %_0.sroa.11.12.insert.mask = or i64 %_0.sroa.13.12.insert.mask, %_0.sroa.13.12.insert.shift
  %_0.sroa.11.12.insert.insert = or i64 %_0.sroa.11.12.insert.mask, %_0.sroa.11.12.insert.ext
  %copy140 = insertvalue %"7runtime6StringS" %3, i64 %_0.sroa.11.12.insert.insert, 1
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6StringS" [ %copy140, %bb3 ], [ %1, %bb ]
  ret %"7runtime6StringS" %_2.0
}

define %"7runtime6String8OptionalV" @"7runtime6String8Optional3mapFF7runtime6StringSE7runtime6StringSE0"(%"7runtime6String8OptionalV" %0, %"7runtime6StringS" (%"7runtime6StringS")* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6String8OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime6String8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt45 = extractvalue [16 x i8] %.elt15, 15
  %.elt15.elt43 = extractvalue [16 x i8] %.elt15, 14
  %.elt15.elt41 = extractvalue [16 x i8] %.elt15, 13
  %.elt15.elt39 = extractvalue [16 x i8] %.elt15, 12
  %.elt15.elt37 = extractvalue [16 x i8] %.elt15, 11
  %.elt15.elt35 = extractvalue [16 x i8] %.elt15, 10
  %.elt15.elt33 = extractvalue [16 x i8] %.elt15, 9
  %.elt15.elt31 = extractvalue [16 x i8] %.elt15, 8
  %.elt15.elt17 = extractvalue [16 x i8] %.elt15, 1
  %_0.sroa.2.5.insert.ext = zext i8 %.elt15.elt17 to i64
  %_0.sroa.2.5.insert.shift = shl nuw nsw i64 %_0.sroa.2.5.insert.ext, 8
  %.elt15.elt = extractvalue [16 x i8] %.elt15, 0
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i64
  %_0.sroa.2.5.insert.insert = or i64 %_0.sroa.2.5.insert.shift, %_0.sroa.2.4.insert.ext
  %.elt15.elt19 = extractvalue [16 x i8] %.elt15, 2
  %_0.sroa.2.6.insert.ext = zext i8 %.elt15.elt19 to i64
  %_0.sroa.2.6.insert.shift = shl nuw nsw i64 %_0.sroa.2.6.insert.ext, 16
  %_0.sroa.2.6.insert.insert = or i64 %_0.sroa.2.5.insert.insert, %_0.sroa.2.6.insert.shift
  %.elt15.elt21 = extractvalue [16 x i8] %.elt15, 3
  %_0.sroa.2.7.insert.ext = zext i8 %.elt15.elt21 to i64
  %_0.sroa.2.7.insert.shift = shl nuw nsw i64 %_0.sroa.2.7.insert.ext, 24
  %_0.sroa.2.7.insert.insert = or i64 %_0.sroa.2.6.insert.insert, %_0.sroa.2.7.insert.shift
  %.elt15.elt23 = extractvalue [16 x i8] %.elt15, 4
  %_0.sroa.2.8.insert.ext = zext i8 %.elt15.elt23 to i64
  %_0.sroa.2.8.insert.shift = shl nuw nsw i64 %_0.sroa.2.8.insert.ext, 32
  %_0.sroa.2.9.insert.mask = or i64 %_0.sroa.2.7.insert.insert, %_0.sroa.2.8.insert.shift
  %.elt15.elt25 = extractvalue [16 x i8] %.elt15, 5
  %_0.sroa.2.9.insert.ext = zext i8 %.elt15.elt25 to i64
  %_0.sroa.2.9.insert.shift = shl nuw nsw i64 %_0.sroa.2.9.insert.ext, 40
  %_0.sroa.2.10.insert.mask.masked = or i64 %_0.sroa.2.9.insert.mask, %_0.sroa.2.9.insert.shift
  %.elt15.elt27 = extractvalue [16 x i8] %.elt15, 6
  %_0.sroa.2.10.insert.ext = zext i8 %.elt15.elt27 to i64
  %_0.sroa.2.10.insert.shift = shl nuw nsw i64 %_0.sroa.2.10.insert.ext, 48
  %.elt15.elt29 = extractvalue [16 x i8] %.elt15, 7
  %_0.sroa.2.11.insert.ext = zext i8 %.elt15.elt29 to i64
  %_0.sroa.2.11.insert.shift = shl nuw i64 %_0.sroa.2.11.insert.ext, 56
  %_0.sroa.2.11.insert.mask = or i64 %_0.sroa.2.11.insert.shift, %_0.sroa.2.10.insert.shift
  %_0.sroa.2.11.insert.insert = or i64 %_0.sroa.2.11.insert.mask, %_0.sroa.2.10.insert.mask.masked
  %2 = inttoptr i64 %_0.sroa.2.11.insert.insert to i8*
  %3 = insertvalue %"7runtime6StringS" undef, i8* %2, 0
  %_0.sroa.19.12.insert.ext = zext i8 %.elt15.elt45 to i64
  %_0.sroa.19.12.insert.shift = shl nuw i64 %_0.sroa.19.12.insert.ext, 56
  %_0.sroa.18.12.insert.ext = zext i8 %.elt15.elt43 to i64
  %_0.sroa.18.12.insert.shift = shl nuw nsw i64 %_0.sroa.18.12.insert.ext, 48
  %_0.sroa.18.12.insert.insert = or i64 %_0.sroa.19.12.insert.shift, %_0.sroa.18.12.insert.shift
  %_0.sroa.17.12.insert.ext = zext i8 %.elt15.elt41 to i64
  %_0.sroa.17.12.insert.shift = shl nuw nsw i64 %_0.sroa.17.12.insert.ext, 40
  %_0.sroa.17.12.insert.insert = or i64 %_0.sroa.18.12.insert.insert, %_0.sroa.17.12.insert.shift
  %_0.sroa.16.12.insert.ext = zext i8 %.elt15.elt39 to i64
  %_0.sroa.16.12.insert.shift = shl nuw nsw i64 %_0.sroa.16.12.insert.ext, 32
  %_0.sroa.16.12.insert.insert = or i64 %_0.sroa.17.12.insert.insert, %_0.sroa.16.12.insert.shift
  %_0.sroa.15.12.insert.ext = zext i8 %.elt15.elt37 to i64
  %_0.sroa.15.12.insert.shift = shl nuw nsw i64 %_0.sroa.15.12.insert.ext, 24
  %_0.sroa.15.12.insert.insert = or i64 %_0.sroa.16.12.insert.insert, %_0.sroa.15.12.insert.shift
  %_0.sroa.14.12.insert.ext = zext i8 %.elt15.elt35 to i64
  %_0.sroa.14.12.insert.shift = shl nuw nsw i64 %_0.sroa.14.12.insert.ext, 16
  %_0.sroa.13.12.insert.ext = zext i8 %.elt15.elt33 to i64
  %_0.sroa.13.12.insert.shift = shl nuw nsw i64 %_0.sroa.13.12.insert.ext, 8
  %_0.sroa.13.12.insert.mask = or i64 %_0.sroa.15.12.insert.insert, %_0.sroa.14.12.insert.shift
  %_0.sroa.11.12.insert.ext = zext i8 %.elt15.elt31 to i64
  %_0.sroa.11.12.insert.mask = or i64 %_0.sroa.13.12.insert.mask, %_0.sroa.13.12.insert.shift
  %_0.sroa.11.12.insert.insert = or i64 %_0.sroa.11.12.insert.mask, %_0.sroa.11.12.insert.ext
  %copy184 = insertvalue %"7runtime6StringS" %3, i64 %_0.sroa.11.12.insert.insert, 1
  %call = tail call %"7runtime6StringS" %1(%"7runtime6StringS" %copy184)
  %call.elt = extractvalue %"7runtime6StringS" %call, 0
  %call.elt86 = extractvalue %"7runtime6StringS" %call, 1
  %_3.sroa.11.12.extract.trunc = trunc i64 %call.elt86 to i8
  %_3.sroa.13.12.extract.shift = lshr i64 %call.elt86, 8
  %_3.sroa.13.12.extract.trunc = trunc i64 %_3.sroa.13.12.extract.shift to i8
  %_3.sroa.14.12.extract.shift = lshr i64 %call.elt86, 16
  %_3.sroa.14.12.extract.trunc = trunc i64 %_3.sroa.14.12.extract.shift to i8
  %_3.sroa.15.12.extract.shift = lshr i64 %call.elt86, 24
  %_3.sroa.15.12.extract.trunc = trunc i64 %_3.sroa.15.12.extract.shift to i8
  %_3.sroa.16.12.extract.shift = lshr i64 %call.elt86, 32
  %_3.sroa.16.12.extract.trunc = trunc i64 %_3.sroa.16.12.extract.shift to i8
  %_3.sroa.17.12.extract.shift = lshr i64 %call.elt86, 40
  %_3.sroa.17.12.extract.trunc = trunc i64 %_3.sroa.17.12.extract.shift to i8
  %_3.sroa.18.12.extract.shift = lshr i64 %call.elt86, 48
  %_3.sroa.18.12.extract.trunc = trunc i64 %_3.sroa.18.12.extract.shift to i8
  %_3.sroa.19.12.extract.shift = lshr i64 %call.elt86, 56
  %_3.sroa.19.12.extract.trunc = trunc i64 %_3.sroa.19.12.extract.shift to i8
  %4 = ptrtoint i8* %call.elt to i64
  %_3.sroa.2.4.extract.trunc = trunc i64 %4 to i8
  %5 = insertvalue [16 x i8] undef, i8 %_3.sroa.2.4.extract.trunc, 0
  %_3.sroa.2.5.extract.shift = lshr i64 %4, 8
  %_3.sroa.2.5.extract.trunc = trunc i64 %_3.sroa.2.5.extract.shift to i8
  %6 = insertvalue [16 x i8] %5, i8 %_3.sroa.2.5.extract.trunc, 1
  %_3.sroa.2.6.extract.shift = lshr i64 %4, 16
  %_3.sroa.2.6.extract.trunc = trunc i64 %_3.sroa.2.6.extract.shift to i8
  %7 = insertvalue [16 x i8] %6, i8 %_3.sroa.2.6.extract.trunc, 2
  %_3.sroa.2.7.extract.shift = lshr i64 %4, 24
  %_3.sroa.2.7.extract.trunc = trunc i64 %_3.sroa.2.7.extract.shift to i8
  %8 = insertvalue [16 x i8] %7, i8 %_3.sroa.2.7.extract.trunc, 3
  %_3.sroa.2.8.extract.shift = lshr i64 %4, 32
  %_3.sroa.2.8.extract.trunc = trunc i64 %_3.sroa.2.8.extract.shift to i8
  %9 = insertvalue [16 x i8] %8, i8 %_3.sroa.2.8.extract.trunc, 4
  %_3.sroa.2.9.extract.shift = lshr i64 %4, 40
  %_3.sroa.2.9.extract.trunc = trunc i64 %_3.sroa.2.9.extract.shift to i8
  %10 = insertvalue [16 x i8] %9, i8 %_3.sroa.2.9.extract.trunc, 5
  %_3.sroa.2.10.extract.shift = lshr i64 %4, 48
  %_3.sroa.2.10.extract.trunc = trunc i64 %_3.sroa.2.10.extract.shift to i8
  %11 = insertvalue [16 x i8] %10, i8 %_3.sroa.2.10.extract.trunc, 6
  %_3.sroa.2.11.extract.shift = lshr i64 %4, 56
  %_3.sroa.2.11.extract.trunc = trunc i64 %_3.sroa.2.11.extract.shift to i8
  %12 = insertvalue [16 x i8] %11, i8 %_3.sroa.2.11.extract.trunc, 7
  %13 = insertvalue [16 x i8] %12, i8 %_3.sroa.11.12.extract.trunc, 8
  %14 = insertvalue [16 x i8] %13, i8 %_3.sroa.13.12.extract.trunc, 9
  %15 = insertvalue [16 x i8] %14, i8 %_3.sroa.14.12.extract.trunc, 10
  %16 = insertvalue [16 x i8] %15, i8 %_3.sroa.15.12.extract.trunc, 11
  %17 = insertvalue [16 x i8] %16, i8 %_3.sroa.16.12.extract.trunc, 12
  %18 = insertvalue [16 x i8] %17, i8 %_3.sroa.17.12.extract.trunc, 13
  %19 = insertvalue [16 x i8] %18, i8 %_3.sroa.18.12.extract.trunc, 14
  %copy8.unpack90122 = insertvalue [16 x i8] %19, i8 %_3.sroa.19.12.extract.trunc, 15
  %copy891 = insertvalue %"7runtime6String8OptionalV" { i32 1, {} undef, [16 x i8] undef }, [16 x i8] %copy8.unpack90122, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6String8OptionalV" [ %copy891, %bb3 ], [ { i32 0, {} undef, [16 x i8] undef }, %bb ]
  ret %"7runtime6String8OptionalV" %_2.0
}

define %"7runtime6String8OptionalV" @"7runtime6String8Optional7flatMapFF7runtime6StringSE7runtime6String8OptionalVE0"(%"7runtime6String8OptionalV" %0, %"7runtime6String8OptionalV" (%"7runtime6StringS")* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6String8OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime6String8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt40 = extractvalue [16 x i8] %.elt10, 15
  %.elt10.elt38 = extractvalue [16 x i8] %.elt10, 14
  %.elt10.elt36 = extractvalue [16 x i8] %.elt10, 13
  %.elt10.elt34 = extractvalue [16 x i8] %.elt10, 12
  %.elt10.elt32 = extractvalue [16 x i8] %.elt10, 11
  %.elt10.elt30 = extractvalue [16 x i8] %.elt10, 10
  %.elt10.elt28 = extractvalue [16 x i8] %.elt10, 9
  %.elt10.elt26 = extractvalue [16 x i8] %.elt10, 8
  %.elt10.elt12 = extractvalue [16 x i8] %.elt10, 1
  %_0.sroa.2.5.insert.ext = zext i8 %.elt10.elt12 to i64
  %_0.sroa.2.5.insert.shift = shl nuw nsw i64 %_0.sroa.2.5.insert.ext, 8
  %.elt10.elt = extractvalue [16 x i8] %.elt10, 0
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i64
  %_0.sroa.2.5.insert.insert = or i64 %_0.sroa.2.5.insert.shift, %_0.sroa.2.4.insert.ext
  %.elt10.elt14 = extractvalue [16 x i8] %.elt10, 2
  %_0.sroa.2.6.insert.ext = zext i8 %.elt10.elt14 to i64
  %_0.sroa.2.6.insert.shift = shl nuw nsw i64 %_0.sroa.2.6.insert.ext, 16
  %_0.sroa.2.6.insert.insert = or i64 %_0.sroa.2.5.insert.insert, %_0.sroa.2.6.insert.shift
  %.elt10.elt16 = extractvalue [16 x i8] %.elt10, 3
  %_0.sroa.2.7.insert.ext = zext i8 %.elt10.elt16 to i64
  %_0.sroa.2.7.insert.shift = shl nuw nsw i64 %_0.sroa.2.7.insert.ext, 24
  %_0.sroa.2.7.insert.insert = or i64 %_0.sroa.2.6.insert.insert, %_0.sroa.2.7.insert.shift
  %.elt10.elt18 = extractvalue [16 x i8] %.elt10, 4
  %_0.sroa.2.8.insert.ext = zext i8 %.elt10.elt18 to i64
  %_0.sroa.2.8.insert.shift = shl nuw nsw i64 %_0.sroa.2.8.insert.ext, 32
  %_0.sroa.2.9.insert.mask = or i64 %_0.sroa.2.7.insert.insert, %_0.sroa.2.8.insert.shift
  %.elt10.elt20 = extractvalue [16 x i8] %.elt10, 5
  %_0.sroa.2.9.insert.ext = zext i8 %.elt10.elt20 to i64
  %_0.sroa.2.9.insert.shift = shl nuw nsw i64 %_0.sroa.2.9.insert.ext, 40
  %_0.sroa.2.10.insert.mask.masked = or i64 %_0.sroa.2.9.insert.mask, %_0.sroa.2.9.insert.shift
  %.elt10.elt22 = extractvalue [16 x i8] %.elt10, 6
  %_0.sroa.2.10.insert.ext = zext i8 %.elt10.elt22 to i64
  %_0.sroa.2.10.insert.shift = shl nuw nsw i64 %_0.sroa.2.10.insert.ext, 48
  %.elt10.elt24 = extractvalue [16 x i8] %.elt10, 7
  %_0.sroa.2.11.insert.ext = zext i8 %.elt10.elt24 to i64
  %_0.sroa.2.11.insert.shift = shl nuw i64 %_0.sroa.2.11.insert.ext, 56
  %_0.sroa.2.11.insert.mask = or i64 %_0.sroa.2.11.insert.shift, %_0.sroa.2.10.insert.shift
  %_0.sroa.2.11.insert.insert = or i64 %_0.sroa.2.11.insert.mask, %_0.sroa.2.10.insert.mask.masked
  %2 = inttoptr i64 %_0.sroa.2.11.insert.insert to i8*
  %3 = insertvalue %"7runtime6StringS" undef, i8* %2, 0
  %_0.sroa.19.12.insert.ext = zext i8 %.elt10.elt40 to i64
  %_0.sroa.19.12.insert.shift = shl nuw i64 %_0.sroa.19.12.insert.ext, 56
  %_0.sroa.18.12.insert.ext = zext i8 %.elt10.elt38 to i64
  %_0.sroa.18.12.insert.shift = shl nuw nsw i64 %_0.sroa.18.12.insert.ext, 48
  %_0.sroa.18.12.insert.insert = or i64 %_0.sroa.19.12.insert.shift, %_0.sroa.18.12.insert.shift
  %_0.sroa.17.12.insert.ext = zext i8 %.elt10.elt36 to i64
  %_0.sroa.17.12.insert.shift = shl nuw nsw i64 %_0.sroa.17.12.insert.ext, 40
  %_0.sroa.17.12.insert.insert = or i64 %_0.sroa.18.12.insert.insert, %_0.sroa.17.12.insert.shift
  %_0.sroa.16.12.insert.ext = zext i8 %.elt10.elt34 to i64
  %_0.sroa.16.12.insert.shift = shl nuw nsw i64 %_0.sroa.16.12.insert.ext, 32
  %_0.sroa.16.12.insert.insert = or i64 %_0.sroa.17.12.insert.insert, %_0.sroa.16.12.insert.shift
  %_0.sroa.15.12.insert.ext = zext i8 %.elt10.elt32 to i64
  %_0.sroa.15.12.insert.shift = shl nuw nsw i64 %_0.sroa.15.12.insert.ext, 24
  %_0.sroa.15.12.insert.insert = or i64 %_0.sroa.16.12.insert.insert, %_0.sroa.15.12.insert.shift
  %_0.sroa.14.12.insert.ext = zext i8 %.elt10.elt30 to i64
  %_0.sroa.14.12.insert.shift = shl nuw nsw i64 %_0.sroa.14.12.insert.ext, 16
  %_0.sroa.13.12.insert.ext = zext i8 %.elt10.elt28 to i64
  %_0.sroa.13.12.insert.shift = shl nuw nsw i64 %_0.sroa.13.12.insert.ext, 8
  %_0.sroa.13.12.insert.mask = or i64 %_0.sroa.15.12.insert.insert, %_0.sroa.14.12.insert.shift
  %_0.sroa.11.12.insert.ext = zext i8 %.elt10.elt26 to i64
  %_0.sroa.11.12.insert.mask = or i64 %_0.sroa.13.12.insert.mask, %_0.sroa.13.12.insert.shift
  %_0.sroa.11.12.insert.insert = or i64 %_0.sroa.11.12.insert.mask, %_0.sroa.11.12.insert.ext
  %copy179 = insertvalue %"7runtime6StringS" %3, i64 %_0.sroa.11.12.insert.insert, 1
  %call = tail call %"7runtime6String8OptionalV" %1(%"7runtime6StringS" %copy179)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime6String8OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [16 x i8] undef }, %bb ]
  ret %"7runtime6String8OptionalV" %_2.0
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char10isBinDigitFE"(i32 %0) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_5 = alloca i32, align 4
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_3, i32 48)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime4Char5equalF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy1)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_5, i32 49)
  %copy3 = load i32, i32* %_5, align 4
  %call4 = tail call i1 @"7runtime4Char5equalF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy3)
  %call7 = tail call i1 @"7runtime4Bool2orF7runtime4BoolS7runtime4BoolSE00"(i1 %call, i1 %call4)
  ret i1 %call7
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char7isDigitFE"(i32 %0) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_5 = alloca i32, align 4
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_3, i32 48)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime4Char13greaterThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy1)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_5, i32 57)
  %copy3 = load i32, i32* %_5, align 4
  %call4 = tail call i1 @"7runtime4Char10lessThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy3)
  %call7 = tail call i1 @"7runtime4Bool3andF7runtime4BoolS7runtime4BoolSE00"(i1 %call, i1 %call4)
  ret i1 %call7
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char10isOctDigitFE"(i32 %0) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_5 = alloca i32, align 4
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_3, i32 48)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime4Char13greaterThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy1)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_5, i32 55)
  %copy3 = load i32, i32* %_5, align 4
  %call4 = tail call i1 @"7runtime4Char10lessThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy3)
  %call7 = tail call i1 @"7runtime4Bool3andF7runtime4BoolS7runtime4BoolSE00"(i1 %call, i1 %call4)
  ret i1 %call7
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char10isHexDigitFE"(i32 %0) local_unnamed_addr #2 {
bb:
  %_3 = alloca i32, align 4
  %_5 = alloca i32, align 4
  %_8 = alloca i32, align 4
  %_10 = alloca i32, align 4
  %_13 = alloca i32, align 4
  %_15 = alloca i32, align 4
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_3, i32 48)
  %copy1 = load i32, i32* %_3, align 4
  %call = tail call i1 @"7runtime4Char13greaterThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy1)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_5, i32 57)
  %copy3 = load i32, i32* %_5, align 4
  %call4 = tail call i1 @"7runtime4Char10lessThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy3)
  %call7 = tail call i1 @"7runtime4Bool3andF7runtime4BoolS7runtime4BoolSE00"(i1 %call, i1 %call4)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_8, i32 97)
  %copy9 = load i32, i32* %_8, align 4
  %call10 = tail call i1 @"7runtime4Char13greaterThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy9)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_10, i32 102)
  %copy12 = load i32, i32* %_10, align 4
  %call13 = tail call i1 @"7runtime4Char10lessThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy12)
  %call16 = tail call i1 @"7runtime4Bool3andF7runtime4BoolS7runtime4BoolSE00"(i1 %call10, i1 %call13)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_13, i32 65)
  %copy18 = load i32, i32* %_13, align 4
  %call19 = tail call i1 @"7runtime4Char13greaterThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy18)
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_15, i32 70)
  %copy21 = load i32, i32* %_15, align 4
  %call22 = tail call i1 @"7runtime4Char10lessThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %copy21)
  %call25 = tail call i1 @"7runtime4Bool3andF7runtime4BoolS7runtime4BoolSE00"(i1 %call19, i1 %call22)
  %call28 = tail call i1 @"7runtime4Bool2orF7runtime4BoolS7runtime4BoolSE00"(i1 %call7, i1 %call16)
  %call31 = tail call i1 @"7runtime4Bool2orF7runtime4BoolS7runtime4BoolSE00"(i1 %call28, i1 %call25)
  ret i1 %call31
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Char4initF7runtime6UInt32SE3raw"(i32* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  store i32 %1, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char5equalF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %eq = icmp eq i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %eq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char8notEqualF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %neq = icmp ne i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %neq)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char11greaterThanF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gt = icmp ugt i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char13greaterThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %gte = icmp uge i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %gte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char8lessThanF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lt = icmp ult i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lt)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define i1 @"7runtime4Char10lessThanEqF7runtime4CharS7runtime4CharSE00"(i32 %0, i32 %1) local_unnamed_addr #2 {
bb:
  %_2 = alloca i1, align 1
  %lte = icmp ule i32 %0, %1
  call void @"7runtime4Bool4initFbE4repr"(i1* nonnull %_2, i1 %lte)
  %copy3 = load i1, i1* %_2, align 1
  ret i1 %copy3
}

define i32 @"7runtime4Char6promptF7runtime6StringSE0"(%"7runtime6StringS" %0) local_unnamed_addr {
bb:
  %_2 = alloca i32, align 4
  tail call void @"7runtime5printF7runtime6StringSE0"(%"7runtime6StringS" %0)
  %call = tail call i32 @readInternalChar()
  call void @"7runtime4Char4initFjE4repr"(i32* nonnull %_2, i32 %call)
  %copy2 = load i32, i32* %_2, align 4
  ret i32 %copy2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn writeonly
define void @"7runtime4Char4initFjE4repr"(i32* nocapture writeonly %0, i32 %1) local_unnamed_addr #2 {
bb:
  store i32 %1, i32* %0, align 4
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i32 @"7runtime4Char8Optional6unwrapF7runtime4CharSE6orElse"(%"7runtime4Char8OptionalV" %0, i32 %1) local_unnamed_addr #0 {
bb:
  %.elt = extractvalue %"7runtime4Char8OptionalV" %0, 0
  %.elt7 = extractvalue %"7runtime4Char8OptionalV" %0, 2
  %.elt7.elt = extractvalue [4 x i8] %.elt7, 0
  %.elt7.elt9 = extractvalue [4 x i8] %.elt7, 1
  %.elt7.elt11 = extractvalue [4 x i8] %.elt7, 2
  %.elt7.elt13 = extractvalue [4 x i8] %.elt7, 3
  %cond = icmp eq i32 %.elt, 1
  %_0.sroa.6.4.insert.ext = zext i8 %.elt7.elt13 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt7.elt11 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt7.elt9 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt7.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %_2.0 = select i1 %cond, i32 %_0.sroa.2.4.insert.insert, i32 %1
  ret i32 %_2.0
}

define %"7runtime4Char8OptionalV" @"7runtime4Char8Optional3mapFF7runtime4CharSE7runtime4CharSE0"(%"7runtime4Char8OptionalV" %0, i32 (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime4Char8OptionalV" %0, 0
  %.elt15 = extractvalue %"7runtime4Char8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt15.elt21 = extractvalue [4 x i8] %.elt15, 3
  %.elt15.elt19 = extractvalue [4 x i8] %.elt15, 2
  %.elt15.elt17 = extractvalue [4 x i8] %.elt15, 1
  %.elt15.elt = extractvalue [4 x i8] %.elt15, 0
  %_0.sroa.6.4.insert.ext = zext i8 %.elt15.elt21 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt15.elt19 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt15.elt17 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt15.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %call = tail call i32 %1(i32 %_0.sroa.2.4.insert.insert)
  %_3.sroa.1.4.extract.trunc = trunc i32 %call to i8
  %_3.sroa.3.4.extract.shift = lshr i32 %call, 8
  %_3.sroa.3.4.extract.trunc = trunc i32 %_3.sroa.3.4.extract.shift to i8
  %_3.sroa.4.4.extract.shift = lshr i32 %call, 16
  %_3.sroa.4.4.extract.trunc = trunc i32 %_3.sroa.4.4.extract.shift to i8
  %_3.sroa.5.4.extract.shift = lshr i32 %call, 24
  %_3.sroa.5.4.extract.trunc = trunc i32 %_3.sroa.5.4.extract.shift to i8
  %2 = insertvalue [4 x i8] undef, i8 %_3.sroa.1.4.extract.trunc, 0
  %3 = insertvalue [4 x i8] %2, i8 %_3.sroa.3.4.extract.trunc, 1
  %4 = insertvalue [4 x i8] %3, i8 %_3.sroa.4.4.extract.trunc, 2
  %copy8.unpack3745 = insertvalue [4 x i8] %4, i8 %_3.sroa.5.4.extract.trunc, 3
  %copy838 = insertvalue %"7runtime4Char8OptionalV" { i32 1, {} undef, [4 x i8] undef }, [4 x i8] %copy8.unpack3745, 2
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4Char8OptionalV" [ %copy838, %bb3 ], [ { i32 0, {} undef, [4 x i8] undef }, %bb ]
  ret %"7runtime4Char8OptionalV" %_2.0
}

define %"7runtime4Char8OptionalV" @"7runtime4Char8Optional7flatMapFF7runtime4CharSE7runtime4Char8OptionalVE0"(%"7runtime4Char8OptionalV" %0, %"7runtime4Char8OptionalV" (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime4Char8OptionalV" %0, 0
  %.elt10 = extractvalue %"7runtime4Char8OptionalV" %0, 2
  %cond = icmp eq i32 %.elt, 1
  br i1 %cond, label %bb3, label %bb5

bb3:                                              ; preds = %bb
  %.elt10.elt16 = extractvalue [4 x i8] %.elt10, 3
  %.elt10.elt14 = extractvalue [4 x i8] %.elt10, 2
  %.elt10.elt12 = extractvalue [4 x i8] %.elt10, 1
  %.elt10.elt = extractvalue [4 x i8] %.elt10, 0
  %_0.sroa.6.4.insert.ext = zext i8 %.elt10.elt16 to i32
  %_0.sroa.6.4.insert.shift = shl nuw i32 %_0.sroa.6.4.insert.ext, 24
  %_0.sroa.5.4.insert.ext = zext i8 %.elt10.elt14 to i32
  %_0.sroa.5.4.insert.shift = shl nuw nsw i32 %_0.sroa.5.4.insert.ext, 16
  %_0.sroa.5.4.insert.insert = or i32 %_0.sroa.6.4.insert.shift, %_0.sroa.5.4.insert.shift
  %_0.sroa.4.4.insert.ext = zext i8 %.elt10.elt12 to i32
  %_0.sroa.4.4.insert.shift = shl nuw nsw i32 %_0.sroa.4.4.insert.ext, 8
  %_0.sroa.4.4.insert.insert = or i32 %_0.sroa.5.4.insert.insert, %_0.sroa.4.4.insert.shift
  %_0.sroa.2.4.insert.ext = zext i8 %.elt10.elt to i32
  %_0.sroa.2.4.insert.insert = or i32 %_0.sroa.4.4.insert.insert, %_0.sroa.2.4.insert.ext
  %call = tail call %"7runtime4Char8OptionalV" %1(i32 %_0.sroa.2.4.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb, %bb3
  %_2.0 = phi %"7runtime4Char8OptionalV" [ %call, %bb3 ], [ { i32 0, {} undef, [4 x i8] undef }, %bb ]
  ret %"7runtime4Char8OptionalV" %_2.0
}

define void @"7runtime5printF7runtime4BoolS7runtime4BoolSE07newline"(i1 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printBool(i1 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime4HalfS7runtime4BoolSE07newline"(half %0, i1 %1) local_unnamed_addr {
bb:
  %fext32 = fpext half %0 to float
  tail call void @printFloat(float %fext32)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime5FloatS7runtime4BoolSE07newline"(float %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printFloat(float %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime6DoubleS7runtime4BoolSE07newline"(double %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printDouble(double %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime3IntS7runtime4BoolSE07newline"(i64 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printInt64(i64 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime5Int64S7runtime4BoolSE07newline"(i64 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printInt64(i64 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime5Int32S7runtime4BoolSE07newline"(i32 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printInt32(i32 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime5Int16S7runtime4BoolSE07newline"(i16 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printInt16(i16 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime4Int8S7runtime4BoolSE07newline"(i8 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printInt8(i8 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime4UIntS7runtime4BoolSE07newline"(i64 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printUInt64(i64 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime6UInt64S7runtime4BoolSE07newline"(i64 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printUInt64(i64 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime6UInt32S7runtime4BoolSE07newline"(i32 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printUInt32(i32 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime6UInt16S7runtime4BoolSE07newline"(i16 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printUInt16(i16 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime5UInt8S7runtime4BoolSE07newline"(i8 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printUInt8(i8 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime6StringS7runtime4BoolSE07newline"(%"7runtime6StringS" %0, i1 %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6StringS" %0, 0
  %.elt5 = extractvalue %"7runtime6StringS" %0, 1
  tail call void @printString(i8* %.elt, i64 %.elt5)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime4CharS7runtime4BoolSE07newline"(i32 %0, i1 %1) local_unnamed_addr {
bb:
  tail call void @printChar(i32 %0)
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb
  tail call void @printLine()
  br label %bb3

bb3:                                              ; preds = %bb, %bb2
  ret void
}

define void @"7runtime5printF7runtime4BoolSE0"(i1 %0) local_unnamed_addr {
bb:
  tail call void @printBool(i1 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime4HalfSE0"(half %0) local_unnamed_addr {
bb:
  %fext32 = fpext half %0 to float
  tail call void @printFloat(float %fext32)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime5FloatSE0"(float %0) local_unnamed_addr {
bb:
  tail call void @printFloat(float %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime6DoubleSE0"(double %0) local_unnamed_addr {
bb:
  tail call void @printDouble(double %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime3IntSE0"(i64 %0) local_unnamed_addr {
bb:
  tail call void @printInt64(i64 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime5Int64SE0"(i64 %0) local_unnamed_addr {
bb:
  tail call void @printInt64(i64 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime5Int32SE0"(i32 %0) local_unnamed_addr {
bb:
  tail call void @printInt32(i32 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime5Int16SE0"(i16 %0) local_unnamed_addr {
bb:
  tail call void @printInt16(i16 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime4Int8SE0"(i8 %0) local_unnamed_addr {
bb:
  tail call void @printInt8(i8 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime4UIntSE0"(i64 %0) local_unnamed_addr {
bb:
  tail call void @printUInt64(i64 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime6UInt64SE0"(i64 %0) local_unnamed_addr {
bb:
  tail call void @printUInt64(i64 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime6UInt32SE0"(i32 %0) local_unnamed_addr {
bb:
  tail call void @printUInt32(i32 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime6UInt16SE0"(i16 %0) local_unnamed_addr {
bb:
  tail call void @printUInt16(i16 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime5UInt8SE0"(i8 %0) local_unnamed_addr {
bb:
  tail call void @printUInt8(i8 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime6StringSE0"(%"7runtime6StringS" %0) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6StringS" %0, 0
  %.elt4 = extractvalue %"7runtime6StringS" %0, 1
  tail call void @printString(i8* %.elt, i64 %.elt4)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime4CharSE0"(i32 %0) local_unnamed_addr {
bb:
  tail call void @printChar(i32 %0)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime6StringS7runtime3IntSE00"(%"7runtime6StringS" %0, i64 %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6StringS" %0, 0
  %.elt5 = extractvalue %"7runtime6StringS" %0, 1
  tail call void @printString(i8* %.elt, i64 %.elt5)
  tail call void @printInt64(i64 %1)
  tail call void @printLine()
  ret void
}

define void @"7runtime5printF7runtime6StringS7runtime5FloatSE00"(%"7runtime6StringS" %0, float %1) local_unnamed_addr {
bb:
  %.elt = extractvalue %"7runtime6StringS" %0, 0
  %.elt5 = extractvalue %"7runtime6StringS" %0, 1
  tail call void @printString(i8* %.elt, i64 %.elt5)
  tail call void @printFloat(float %1)
  tail call void @printLine()
  ret void
}

define void @"7runtime3forF7runtime3Int5RangeSF7runtime3IntSEuE00"(%"7runtime3Int5RangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime3Int5RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime3Int5RangeS" %0, 1
  %call16 = tail call i1 @"7runtime3Int8lessThanF7runtime3IntS7runtime3IntSE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime3Int3addF7runtime3IntS7runtime3IntSE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime3Int8lessThanF7runtime3IntS7runtime3IntSE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime3Int11ClosedRangeSF7runtime3IntSEuE00"(%"7runtime3Int11ClosedRangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime3Int11ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime3Int11ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime3Int10lessThanEqF7runtime3IntS7runtime3IntSE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime3Int4initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime3Int3addF7runtime3IntS7runtime3IntSE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime3Int10lessThanEqF7runtime3IntS7runtime3IntSE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime4UInt5RangeSF7runtime4UIntSEuE00"(%"7runtime4UInt5RangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime4UInt5RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime4UInt5RangeS" %0, 1
  %call16 = tail call i1 @"7runtime4UInt8lessThanF7runtime4UIntS7runtime4UIntSE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime4UInt3addF7runtime4UIntS7runtime4UIntSE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime4UInt8lessThanF7runtime4UIntS7runtime4UIntSE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime4UInt11ClosedRangeSF7runtime4UIntSEuE00"(%"7runtime4UInt11ClosedRangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime4UInt11ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime4UInt11ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime4UInt10lessThanEqF7runtime4UIntS7runtime4UIntSE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime4UInt4initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime4UInt3addF7runtime4UIntS7runtime4UIntSE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime4UInt10lessThanEqF7runtime4UIntS7runtime4UIntSE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime4Int85RangeSF7runtime4Int8SEuE00"(%"7runtime4Int85RangeS" %0, void (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i8, align 1
  %.elt = extractvalue %"7runtime4Int85RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime4Int85RangeS" %0, 1
  %call16 = tail call i1 @"7runtime4Int88lessThanF7runtime4Int8S7runtime4Int8SE00"(i8 %.elt, i8 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i8 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i8 %_2.sroa.0.017)
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_5, i8 1)
  %copy11 = load i8, i8* %_5, align 1
  %call12 = tail call i8 @"7runtime4Int83addF7runtime4Int8S7runtime4Int8SE00"(i8 %_2.sroa.0.017, i8 %copy11)
  %call = tail call i1 @"7runtime4Int88lessThanF7runtime4Int8S7runtime4Int8SE00"(i8 %call12, i8 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime4Int811ClosedRangeSF7runtime4Int8SEuE00"(%"7runtime4Int811ClosedRangeS" %0, void (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i8, align 1
  %.elt = extractvalue %"7runtime4Int811ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime4Int811ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime4Int810lessThanEqF7runtime4Int8S7runtime4Int8SE00"(i8 %.elt, i8 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i8 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i8 %_2.sroa.0.017)
  call void @"7runtime4Int84initFaE4repr"(i8* nonnull %_5, i8 1)
  %copy11 = load i8, i8* %_5, align 1
  %call12 = tail call i8 @"7runtime4Int83addF7runtime4Int8S7runtime4Int8SE00"(i8 %_2.sroa.0.017, i8 %copy11)
  %call = tail call i1 @"7runtime4Int810lessThanEqF7runtime4Int8S7runtime4Int8SE00"(i8 %call12, i8 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5Int165RangeSF7runtime5Int16SEuE00"(%"7runtime5Int165RangeS" %0, void (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i16, align 2
  %.elt = extractvalue %"7runtime5Int165RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5Int165RangeS" %0, 1
  %call16 = tail call i1 @"7runtime5Int168lessThanF7runtime5Int16S7runtime5Int16SE00"(i16 %.elt, i16 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i16 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i16 %_2.sroa.0.017)
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_5, i16 1)
  %copy11 = load i16, i16* %_5, align 2
  %call12 = tail call i16 @"7runtime5Int163addF7runtime5Int16S7runtime5Int16SE00"(i16 %_2.sroa.0.017, i16 %copy11)
  %call = tail call i1 @"7runtime5Int168lessThanF7runtime5Int16S7runtime5Int16SE00"(i16 %call12, i16 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5Int1611ClosedRangeSF7runtime5Int16SEuE00"(%"7runtime5Int1611ClosedRangeS" %0, void (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i16, align 2
  %.elt = extractvalue %"7runtime5Int1611ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5Int1611ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime5Int1610lessThanEqF7runtime5Int16S7runtime5Int16SE00"(i16 %.elt, i16 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i16 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i16 %_2.sroa.0.017)
  call void @"7runtime5Int164initFlE4repr"(i16* nonnull %_5, i16 1)
  %copy11 = load i16, i16* %_5, align 2
  %call12 = tail call i16 @"7runtime5Int163addF7runtime5Int16S7runtime5Int16SE00"(i16 %_2.sroa.0.017, i16 %copy11)
  %call = tail call i1 @"7runtime5Int1610lessThanEqF7runtime5Int16S7runtime5Int16SE00"(i16 %call12, i16 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5Int325RangeSF7runtime5Int32SEuE00"(%"7runtime5Int325RangeS" %0, void (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i32, align 4
  %.elt = extractvalue %"7runtime5Int325RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5Int325RangeS" %0, 1
  %call16 = tail call i1 @"7runtime5Int328lessThanF7runtime5Int32S7runtime5Int32SE00"(i32 %.elt, i32 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i32 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i32 %_2.sroa.0.017)
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_5, i32 1)
  %copy11 = load i32, i32* %_5, align 4
  %call12 = tail call i32 @"7runtime5Int323addF7runtime5Int32S7runtime5Int32SE00"(i32 %_2.sroa.0.017, i32 %copy11)
  %call = tail call i1 @"7runtime5Int328lessThanF7runtime5Int32S7runtime5Int32SE00"(i32 %call12, i32 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5Int3211ClosedRangeSF7runtime5Int32SEuE00"(%"7runtime5Int3211ClosedRangeS" %0, void (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i32, align 4
  %.elt = extractvalue %"7runtime5Int3211ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5Int3211ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime5Int3210lessThanEqF7runtime5Int32S7runtime5Int32SE00"(i32 %.elt, i32 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i32 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i32 %_2.sroa.0.017)
  call void @"7runtime5Int324initFjE4repr"(i32* nonnull %_5, i32 1)
  %copy11 = load i32, i32* %_5, align 4
  %call12 = tail call i32 @"7runtime5Int323addF7runtime5Int32S7runtime5Int32SE00"(i32 %_2.sroa.0.017, i32 %copy11)
  %call = tail call i1 @"7runtime5Int3210lessThanEqF7runtime5Int32S7runtime5Int32SE00"(i32 %call12, i32 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5Int645RangeSF7runtime5Int64SEuE00"(%"7runtime5Int645RangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime5Int645RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5Int645RangeS" %0, 1
  %call16 = tail call i1 @"7runtime5Int648lessThanF7runtime5Int64S7runtime5Int64SE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime5Int643addF7runtime5Int64S7runtime5Int64SE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime5Int648lessThanF7runtime5Int64S7runtime5Int64SE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5Int6411ClosedRangeSF7runtime5Int64SEuE00"(%"7runtime5Int6411ClosedRangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime5Int6411ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5Int6411ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime5Int6410lessThanEqF7runtime5Int64S7runtime5Int64SE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime5Int644initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime5Int643addF7runtime5Int64S7runtime5Int64SE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime5Int6410lessThanEqF7runtime5Int64S7runtime5Int64SE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5UInt85RangeSF7runtime5UInt8SEuE00"(%"7runtime5UInt85RangeS" %0, void (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i8, align 1
  %.elt = extractvalue %"7runtime5UInt85RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5UInt85RangeS" %0, 1
  %call16 = tail call i1 @"7runtime5UInt88lessThanF7runtime5UInt8S7runtime5UInt8SE00"(i8 %.elt, i8 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i8 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i8 %_2.sroa.0.017)
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_5, i8 1)
  %copy11 = load i8, i8* %_5, align 1
  %call12 = tail call i8 @"7runtime5UInt83addF7runtime5UInt8S7runtime5UInt8SE00"(i8 %_2.sroa.0.017, i8 %copy11)
  %call = tail call i1 @"7runtime5UInt88lessThanF7runtime5UInt8S7runtime5UInt8SE00"(i8 %call12, i8 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime5UInt811ClosedRangeSF7runtime5UInt8SEuE00"(%"7runtime5UInt811ClosedRangeS" %0, void (i8)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i8, align 1
  %.elt = extractvalue %"7runtime5UInt811ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime5UInt811ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime5UInt810lessThanEqF7runtime5UInt8S7runtime5UInt8SE00"(i8 %.elt, i8 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i8 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i8 %_2.sroa.0.017)
  call void @"7runtime5UInt84initFaE4repr"(i8* nonnull %_5, i8 1)
  %copy11 = load i8, i8* %_5, align 1
  %call12 = tail call i8 @"7runtime5UInt83addF7runtime5UInt8S7runtime5UInt8SE00"(i8 %_2.sroa.0.017, i8 %copy11)
  %call = tail call i1 @"7runtime5UInt810lessThanEqF7runtime5UInt8S7runtime5UInt8SE00"(i8 %call12, i8 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime6UInt165RangeSF7runtime6UInt16SEuE00"(%"7runtime6UInt165RangeS" %0, void (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i16, align 2
  %.elt = extractvalue %"7runtime6UInt165RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime6UInt165RangeS" %0, 1
  %call16 = tail call i1 @"7runtime6UInt168lessThanF7runtime6UInt16S7runtime6UInt16SE00"(i16 %.elt, i16 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i16 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i16 %_2.sroa.0.017)
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_5, i16 1)
  %copy11 = load i16, i16* %_5, align 2
  %call12 = tail call i16 @"7runtime6UInt163addF7runtime6UInt16S7runtime6UInt16SE00"(i16 %_2.sroa.0.017, i16 %copy11)
  %call = tail call i1 @"7runtime6UInt168lessThanF7runtime6UInt16S7runtime6UInt16SE00"(i16 %call12, i16 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime6UInt1611ClosedRangeSF7runtime6UInt16SEuE00"(%"7runtime6UInt1611ClosedRangeS" %0, void (i16)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i16, align 2
  %.elt = extractvalue %"7runtime6UInt1611ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime6UInt1611ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime6UInt1610lessThanEqF7runtime6UInt16S7runtime6UInt16SE00"(i16 %.elt, i16 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i16 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i16 %_2.sroa.0.017)
  call void @"7runtime6UInt164initFlE4repr"(i16* nonnull %_5, i16 1)
  %copy11 = load i16, i16* %_5, align 2
  %call12 = tail call i16 @"7runtime6UInt163addF7runtime6UInt16S7runtime6UInt16SE00"(i16 %_2.sroa.0.017, i16 %copy11)
  %call = tail call i1 @"7runtime6UInt1610lessThanEqF7runtime6UInt16S7runtime6UInt16SE00"(i16 %call12, i16 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime6UInt325RangeSF7runtime6UInt32SEuE00"(%"7runtime6UInt325RangeS" %0, void (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i32, align 4
  %.elt = extractvalue %"7runtime6UInt325RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime6UInt325RangeS" %0, 1
  %call16 = tail call i1 @"7runtime6UInt328lessThanF7runtime6UInt32S7runtime6UInt32SE00"(i32 %.elt, i32 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i32 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i32 %_2.sroa.0.017)
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_5, i32 1)
  %copy11 = load i32, i32* %_5, align 4
  %call12 = tail call i32 @"7runtime6UInt323addF7runtime6UInt32S7runtime6UInt32SE00"(i32 %_2.sroa.0.017, i32 %copy11)
  %call = tail call i1 @"7runtime6UInt328lessThanF7runtime6UInt32S7runtime6UInt32SE00"(i32 %call12, i32 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime6UInt3211ClosedRangeSF7runtime6UInt32SEuE00"(%"7runtime6UInt3211ClosedRangeS" %0, void (i32)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i32, align 4
  %.elt = extractvalue %"7runtime6UInt3211ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime6UInt3211ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime6UInt3210lessThanEqF7runtime6UInt32S7runtime6UInt32SE00"(i32 %.elt, i32 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i32 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i32 %_2.sroa.0.017)
  call void @"7runtime6UInt324initFjE4repr"(i32* nonnull %_5, i32 1)
  %copy11 = load i32, i32* %_5, align 4
  %call12 = tail call i32 @"7runtime6UInt323addF7runtime6UInt32S7runtime6UInt32SE00"(i32 %_2.sroa.0.017, i32 %copy11)
  %call = tail call i1 @"7runtime6UInt3210lessThanEqF7runtime6UInt32S7runtime6UInt32SE00"(i32 %call12, i32 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime6UInt645RangeSF7runtime6UInt64SEuE00"(%"7runtime6UInt645RangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime6UInt645RangeS" %0, 0
  %.elt14 = extractvalue %"7runtime6UInt645RangeS" %0, 1
  %call16 = tail call i1 @"7runtime6UInt648lessThanF7runtime6UInt64S7runtime6UInt64SE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime6UInt643addF7runtime6UInt64S7runtime6UInt64SE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime6UInt648lessThanF7runtime6UInt64S7runtime6UInt64SE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

define void @"7runtime3forF7runtime6UInt6411ClosedRangeSF7runtime6UInt64SEuE00"(%"7runtime6UInt6411ClosedRangeS" %0, void (i64)* nocapture readonly %1) local_unnamed_addr {
bb:
  %_5 = alloca i64, align 8
  %.elt = extractvalue %"7runtime6UInt6411ClosedRangeS" %0, 0
  %.elt14 = extractvalue %"7runtime6UInt6411ClosedRangeS" %0, 1
  %call16 = tail call i1 @"7runtime6UInt6410lessThanEqF7runtime6UInt64S7runtime6UInt64SE00"(i64 %.elt, i64 %.elt14)
  br i1 %call16, label %bb4, label %bb7

bb4:                                              ; preds = %bb, %bb4
  %_2.sroa.0.017 = phi i64 [ %call12, %bb4 ], [ %.elt, %bb ]
  tail call void %1(i64 %_2.sroa.0.017)
  call void @"7runtime6UInt644initFiE4repr"(i64* nonnull %_5, i64 1)
  %copy11 = load i64, i64* %_5, align 8
  %call12 = tail call i64 @"7runtime6UInt643addF7runtime6UInt64S7runtime6UInt64SE00"(i64 %_2.sroa.0.017, i64 %copy11)
  %call = tail call i1 @"7runtime6UInt6410lessThanEqF7runtime6UInt64S7runtime6UInt64SE00"(i64 %call12, i64 %.elt14)
  br i1 %call, label %bb4, label %bb7

bb7:                                              ; preds = %bb4, %bb
  ret void
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind readnone willreturn }
attributes #1 = { nofree norecurse nosync nounwind writeonly }
attributes #2 = { mustprogress nofree norecurse nosync nounwind willreturn writeonly }
