; ModuleID = 'probe1.871f7ce21f6bf451-cgu.0'
source_filename = "probe1.871f7ce21f6bf451-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, %"core::option::Option<&[core::fmt::rt::Placeholder]>" }
%"core::option::Option<&[core::fmt::rt::Placeholder]>" = type { ptr, [1 x i64] }
%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8>", i64 }
%"alloc::raw_vec::RawVec<u8>" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::alloc::layout::Layout" = type { i64, i64 }
%"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>" = type { i64, [1 x i64] }
%"core::option::Option<&str>" = type { ptr, [1 x i64] }
%"core::ptr::metadata::PtrComponents<[u8]>" = type { ptr, i64 }
%"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] }
%"core::ptr::non_null::NonNull<[u8]>" = type { { ptr, i64 } }
%"core::result::Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>" = type { ptr, [1 x i64] }
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>" = type { [1 x i64], i64, [1 x i64] }
%"core::fmt::rt::Argument<'_>" = type { ptr, ptr }

@0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_91c7fa63c3cfeaa3c795652d5cf060e4 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc_af99043bc04c419363a7f04d23183506 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_91c7fa63c3cfeaa3c795652d5cf060e4, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc_513570631223a12912d85da2bec3b15a = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc_a74f0dda6f0d17444677f6082c36721f = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/25ef9e3d85d934b27d9dada2f9dd52b1dc63bb04/library/core/src/fmt/mod.rs" }>, align 1
@alloc_33b9f5b735a3983a77ab40741e2d9148 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a74f0dda6f0d17444677f6082c36721f, [16 x i8] c"K\00\00\00\00\00\00\00M\01\00\00\0D\00\00\00" }>, align 8
@alloc_b81014ce45aa872ca615246e4b54f5ff = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a74f0dda6f0d17444677f6082c36721f, [16 x i8] c"K\00\00\00\00\00\00\00C\01\00\00\0D\00\00\00" }>, align 8
@alloc_07a843ad5aaabfde28e6b9fa212fdc43 = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/25ef9e3d85d934b27d9dada2f9dd52b1dc63bb04/library/core/src/alloc/layout.rs" }>, align 1
@alloc_22ae9becb653bf1d032abd85d5f955e3 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_07a843ad5aaabfde28e6b9fa212fdc43, [16 x i8] c"P\00\00\00\00\00\00\00\C1\01\00\00)\00\00\00" }>, align 8
@str.0 = internal unnamed_addr constant [25 x i8] c"attempt to divide by zero"
@__rust_no_alloc_shim_is_unstable = external global i8
@alloc_b99730e73100e73a81f4fbfe74b3821d = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_513570631223a12912d85da2bec3b15a, [8 x i8] zeroinitializer }>, align 8
@alloc_53973d2fe29b4adba8bb7390b5678745 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 8

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h82784979634012efE(ptr sret(%"core::fmt::Arguments<'_>") align 8 %_0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_14 = alloca %"core::fmt::Arguments<'_>", align 8
  %_9 = alloca %"core::fmt::Arguments<'_>", align 8
  %_3 = icmp ult i64 %pieces.1, %args.1
  br i1 %_3, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %_7 = add i64 %args.1, 1
  %_6 = icmp ugt i64 %pieces.1, %_7
  br i1 %_6, label %bb2, label %bb3

bb2:                                              ; preds = %bb1, %start
  br i1 false, label %bb4, label %bb6

bb3:                                              ; preds = %bb1
  store ptr %pieces.0, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %pieces.1, ptr %0, align 8
  %1 = load ptr, ptr @0, align 8, !align !3, !noundef !4
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %3 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 2
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 1
  store ptr %args.0, ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 %args.1, ptr %6, align 8
  ret void

bb6:                                              ; preds = %bb2
  store ptr @alloc_af99043bc04c419363a7f04d23183506, ptr %_9, align 8
  %7 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 1, ptr %7, align 8
  %8 = load ptr, ptr @0, align 8, !align !3, !noundef !4
  %9 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %10 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_9, i32 0, i32 2
  store ptr %8, ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %10, i64 8
  store i64 %9, ptr %11, align 8
  %12 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_9, i32 0, i32 1
  store ptr @alloc_513570631223a12912d85da2bec3b15a, ptr %12, align 8
  %13 = getelementptr inbounds i8, ptr %12, i64 8
  store i64 0, ptr %13, align 8
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17ha6effc2775a0749cE(ptr align 8 %_9, ptr align 8 @alloc_33b9f5b735a3983a77ab40741e2d9148) #12
  unreachable

bb4:                                              ; preds = %bb2
; call core::fmt::Arguments::new_const
  call void @_ZN4core3fmt9Arguments9new_const17hc5b42cf7ec926735E(ptr sret(%"core::fmt::Arguments<'_>") align 8 %_14, ptr align 8 @alloc_af99043bc04c419363a7f04d23183506, i64 1)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17ha6effc2775a0749cE(ptr align 8 %_14, ptr align 8 @alloc_b81014ce45aa872ca615246e4b54f5ff) #12
  unreachable
}

; core::fmt::Arguments::new_const
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments9new_const17hc5b42cf7ec926735E(ptr sret(%"core::fmt::Arguments<'_>") align 8 %_0, ptr align 8 %pieces.0, i64 %pieces.1) unnamed_addr #0 {
start:
  %_5 = alloca %"core::fmt::Arguments<'_>", align 8
  %_2 = icmp ugt i64 %pieces.1, 1
  br i1 %_2, label %bb1, label %bb3

bb3:                                              ; preds = %start
  store ptr %pieces.0, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %pieces.1, ptr %0, align 8
  %1 = load ptr, ptr @0, align 8, !align !3, !noundef !4
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %3 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 2
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 1
  store ptr @alloc_513570631223a12912d85da2bec3b15a, ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 0, ptr %6, align 8
  ret void

bb1:                                              ; preds = %start
; call core::fmt::Arguments::new_const
  call void @_ZN4core3fmt9Arguments9new_const17hc5b42cf7ec926735E(ptr sret(%"core::fmt::Arguments<'_>") align 8 %_5, ptr align 8 @alloc_af99043bc04c419363a7f04d23183506, i64 1)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17ha6effc2775a0749cE(ptr align 8 %_5, ptr align 8 @alloc_b81014ce45aa872ca615246e4b54f5ff) #12
  unreachable
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h85d7efbc48b53390E(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 1 %0, i64 %1) unnamed_addr #0 {
start:
  %_2 = alloca { { ptr, i64 } }, align 8
  store ptr %0, ptr %_2, align 8
  %2 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 %1, ptr %2, align 8
  %3 = load ptr, ptr %_2, align 8, !nonnull !4, !align !5, !noundef !4
  %4 = getelementptr inbounds i8, ptr %_2, i64 8
  %5 = load i64, ptr %4, align 8, !noundef !4
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h36c97d6dc41af119E"(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 1 %3, i64 %5)
  ret void
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: nonlazybind uwtable
define void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4ca7eb99b039b66aE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h9e3d5e3ddcd24775E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: nonlazybind uwtable
define void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h9e3d5e3ddcd24775E"(ptr align 8 %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32, [1 x i32] }, align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h54cf3ba6880e5fa7E"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17he163e3357e6b581dE"(ptr align 8 %_1) #13
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17he163e3357e6b581dE"(ptr align 8 %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17hbacfddf1bcf21a1eE() #14
  unreachable

bb1:                                              ; preds = %bb3
  %8 = load ptr, ptr %0, align 8, !noundef !4
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  %10 = load i32, ptr %9, align 8, !noundef !4
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1
  resume { ptr, i32 } %12
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: nonlazybind uwtable
define void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17he163e3357e6b581dE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h076e862acfb0ec91E"(ptr align 8 %_1)
  ret void
}

; core::alloc::layout::Layout::array::inner
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h2f13b444b6013ad3E(i64 %element_size, i64 %align, i64 %n) unnamed_addr #0 {
start:
  %_18 = alloca i64, align 8
  %_13 = alloca i64, align 8
  %_9 = alloca %"core::alloc::layout::Layout", align 8
  %_0 = alloca %"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>", align 8
  %0 = icmp eq i64 %element_size, 0
  br i1 %0, label %bb5, label %bb1

bb5:                                              ; preds = %bb4, %start
  %array_size = mul nuw i64 %element_size, %n
  store i64 %align, ptr %_18, align 8
  %_19 = load i64, ptr %_18, align 8, !range !6, !noundef !4
  %_20 = icmp uge i64 %_19, 1
  %_21 = icmp ule i64 %_19, -9223372036854775808
  %_22 = and i1 %_20, %_21
  call void @llvm.assume(i1 %_22)
  %1 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 %array_size, ptr %1, align 8
  store i64 %_19, ptr %_9, align 8
  %2 = load i64, ptr %_9, align 8, !range !6, !noundef !4
  %3 = getelementptr inbounds i8, ptr %_9, i64 8
  %4 = load i64, ptr %3, align 8, !noundef !4
  store i64 %2, ptr %_0, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %4, ptr %5, align 8
  br label %bb6

bb1:                                              ; preds = %start
  store i64 %align, ptr %_13, align 8
  %_14 = load i64, ptr %_13, align 8, !range !6, !noundef !4
  %_15 = icmp uge i64 %_14, 1
  %_16 = icmp ule i64 %_14, -9223372036854775808
  %_17 = and i1 %_15, %_16
  call void @llvm.assume(i1 %_17)
  %_11 = sub i64 %_14, 1
  %_6 = sub i64 9223372036854775807, %_11
  %_7 = icmp eq i64 %element_size, 0
  %6 = call i1 @llvm.expect.i1(i1 %_7, i1 false)
  br i1 %6, label %panic, label %bb2

bb2:                                              ; preds = %bb1
  %_5 = udiv i64 %_6, %element_size
  %_4 = icmp ugt i64 %n, %_5
  br i1 %_4, label %bb3, label %bb4

panic:                                            ; preds = %bb1
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h44790a89027c670fE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_22ae9becb653bf1d032abd85d5f955e3) #12
  unreachable

bb4:                                              ; preds = %bb2
  br label %bb5

bb3:                                              ; preds = %bb2
  %7 = load i64, ptr @0, align 8, !range !7, !noundef !4
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %7, ptr %_0, align 8
  %9 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %8, ptr %9, align 8
  br label %bb6

bb6:                                              ; preds = %bb3, %bb5
  %10 = load i64, ptr %_0, align 8, !range !7, !noundef !4
  %11 = getelementptr inbounds i8, ptr %_0, i64 8
  %12 = load i64, ptr %11, align 8
  %13 = insertvalue { i64, i64 } poison, i64 %10, 0
  %14 = insertvalue { i64, i64 } %13, i64 %12, 1
  ret { i64, i64 } %14
}

; core::option::Option<T>::map_or_else
; Function Attrs: inlinehint nonlazybind uwtable
define void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h1a9cc892b713a190E"(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 1 %0, i64 %1, ptr align 8 %default) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %2 = alloca { ptr, i32, [1 x i32] }, align 8
  %_10 = alloca i8, align 1
  %_9 = alloca i8, align 1
  %_7 = alloca { { ptr, i64 } }, align 8
  %self = alloca %"core::option::Option<&str>", align 8
  store ptr %0, ptr %self, align 8
  %3 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %1, ptr %3, align 8
  store i8 1, ptr %_10, align 1
  store i8 1, ptr %_9, align 1
  %4 = load ptr, ptr %self, align 8, !noundef !4
  %5 = ptrtoint ptr %4 to i64
  %6 = icmp eq i64 %5, 0
  %_4 = select i1 %6, i64 0, i64 1
  %7 = icmp eq i64 %_4, 0
  br i1 %7, label %bb1, label %bb3

bb1:                                              ; preds = %start
  store i8 0, ptr %_10, align 1
; invoke alloc::fmt::format::{{closure}}
  invoke void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17hb6b07ebbc59dc1d4E"(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 8 %default)
          to label %bb5 unwind label %cleanup

bb3:                                              ; preds = %start
  %t.0 = load ptr, ptr %self, align 8, !nonnull !4, !align !5, !noundef !4
  %8 = getelementptr inbounds i8, ptr %self, i64 8
  %t.1 = load i64, ptr %8, align 8, !noundef !4
  store i8 0, ptr %_9, align 1
  store ptr %t.0, ptr %_7, align 8
  %9 = getelementptr inbounds i8, ptr %_7, i64 8
  store i64 %t.1, ptr %9, align 8
  %10 = load ptr, ptr %_7, align 8, !nonnull !4, !align !5, !noundef !4
  %11 = getelementptr inbounds i8, ptr %_7, i64 8
  %12 = load i64, ptr %11, align 8, !noundef !4
; invoke core::ops::function::FnOnce::call_once
  invoke void @_ZN4core3ops8function6FnOnce9call_once17h85d7efbc48b53390E(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 1 %10, i64 %12)
          to label %bb4 unwind label %cleanup

bb14:                                             ; preds = %cleanup
  %13 = load i8, ptr %_9, align 1, !range !8, !noundef !4
  %14 = trunc i8 %13 to i1
  br i1 %14, label %bb13, label %bb8

cleanup:                                          ; preds = %bb3, %bb1
  %15 = landingpad { ptr, i32 }
          cleanup
  %16 = extractvalue { ptr, i32 } %15, 0
  %17 = extractvalue { ptr, i32 } %15, 1
  store ptr %16, ptr %2, align 8
  %18 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %17, ptr %18, align 8
  br label %bb14

bb5:                                              ; preds = %bb1
  br label %bb11

bb11:                                             ; preds = %bb4, %bb5
  %19 = load i8, ptr %_9, align 1, !range !8, !noundef !4
  %20 = trunc i8 %19 to i1
  br i1 %20, label %bb10, label %bb6

bb4:                                              ; preds = %bb3
  br label %bb11

bb6:                                              ; preds = %bb10, %bb11
  %21 = load i8, ptr %_10, align 1, !range !8, !noundef !4
  %22 = trunc i8 %21 to i1
  br i1 %22, label %bb12, label %bb7

bb10:                                             ; preds = %bb11
  br label %bb6

bb7:                                              ; preds = %bb12, %bb6
  ret void

bb12:                                             ; preds = %bb6
  br label %bb7

bb8:                                              ; preds = %bb13, %bb14
  %23 = load i8, ptr %_10, align 1, !range !8, !noundef !4
  %24 = trunc i8 %23 to i1
  br i1 %24, label %bb15, label %bb9

bb13:                                             ; preds = %bb14
  br label %bb8

bb9:                                              ; preds = %bb15, %bb8
  %25 = load ptr, ptr %2, align 8, !noundef !4
  %26 = getelementptr inbounds i8, ptr %2, i64 8
  %27 = load i32, ptr %26, align 8, !noundef !4
  %28 = insertvalue { ptr, i32 } poison, ptr %25, 0
  %29 = insertvalue { ptr, i32 } %28, i32 %27, 1
  resume { ptr, i32 } %29

bb15:                                             ; preds = %bb8
  br label %bb9

bb2:                                              ; No predecessors!
  unreachable
}

; alloc::fmt::format
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN5alloc3fmt6format17h05bcfe910207afbfE(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 8 %args) unnamed_addr #0 {
start:
  %_4 = alloca ptr, align 8
  %_2 = alloca %"core::option::Option<&str>", align 8
  %_6.0 = load ptr, ptr %args, align 8, !nonnull !4, !align !3, !noundef !4
  %0 = getelementptr inbounds i8, ptr %args, i64 8
  %_6.1 = load i64, ptr %0, align 8, !noundef !4
  %1 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %args, i32 0, i32 1
  %_7.0 = load ptr, ptr %1, align 8, !nonnull !4, !align !3, !noundef !4
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  %_7.1 = load i64, ptr %2, align 8, !noundef !4
  %3 = icmp eq i64 %_6.1, 0
  br i1 %3, label %bb3, label %bb5

bb3:                                              ; preds = %start
  %4 = icmp eq i64 %_7.1, 0
  br i1 %4, label %bb7, label %bb4

bb5:                                              ; preds = %start
  %5 = icmp eq i64 %_6.1, 1
  br i1 %5, label %bb6, label %bb4

bb7:                                              ; preds = %bb3
  store ptr @alloc_513570631223a12912d85da2bec3b15a, ptr %_2, align 8
  %6 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 0, ptr %6, align 8
  br label %bb2

bb4:                                              ; preds = %bb6, %bb5, %bb3
  %7 = load ptr, ptr @0, align 8, !align !5, !noundef !4
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %7, ptr %_2, align 8
  %9 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 %8, ptr %9, align 8
  br label %bb2

bb2:                                              ; preds = %bb4, %bb8, %bb7
  store ptr %args, ptr %_4, align 8
  %10 = load ptr, ptr %_2, align 8, !align !5, !noundef !4
  %11 = getelementptr inbounds i8, ptr %_2, i64 8
  %12 = load i64, ptr %11, align 8
  %13 = load ptr, ptr %_4, align 8, !nonnull !4, !align !3, !noundef !4
; call core::option::Option<T>::map_or_else
  call void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h1a9cc892b713a190E"(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 1 %10, i64 %12, ptr align 8 %13)
  ret void

bb6:                                              ; preds = %bb5
  %14 = icmp eq i64 %_7.1, 0
  br i1 %14, label %bb8, label %bb4

bb8:                                              ; preds = %bb6
  %s = getelementptr inbounds [0 x { ptr, i64 }], ptr %_6.0, i64 0, i64 0
  %15 = getelementptr inbounds [0 x { ptr, i64 }], ptr %_6.0, i64 0, i64 0
  %_13.0 = load ptr, ptr %15, align 8, !nonnull !4, !align !5, !noundef !4
  %16 = getelementptr inbounds i8, ptr %15, i64 8
  %_13.1 = load i64, ptr %16, align 8, !noundef !4
  store ptr %_13.0, ptr %_2, align 8
  %17 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 %_13.1, ptr %17, align 8
  br label %bb2
}

; alloc::fmt::format::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17hb6b07ebbc59dc1d4E"(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 8 %_1) unnamed_addr #0 {
start:
  %_2 = alloca %"core::fmt::Arguments<'_>", align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_2, ptr align 8 %_1, i64 48, i1 false)
; call alloc::fmt::format::format_inner
  call void @_ZN5alloc3fmt6format12format_inner17h20bbaee2ca87fbecE(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 8 %_2)
  ret void
}

; alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h36c97d6dc41af119E"(ptr sret(%"alloc::string::String") align 8 %_0, ptr align 1 %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %v = alloca %"alloc::vec::Vec<u8>", align 8
  %bytes = alloca %"alloc::vec::Vec<u8>", align 8
; call alloc::raw_vec::RawVec<T,A>::allocate_in
  %0 = call { i64, ptr } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hf6de168e2ebec739E"(i64 %self.1, i1 zeroext false)
  %_10.0 = extractvalue { i64, ptr } %0, 0
  %_10.1 = extractvalue { i64, ptr } %0, 1
  store i64 %_10.0, ptr %v, align 8
  %1 = getelementptr inbounds i8, ptr %v, i64 8
  store ptr %_10.1, ptr %1, align 8
  %2 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %v, i32 0, i32 1
  store i64 0, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %v, i64 8
  %self = load ptr, ptr %3, align 8, !nonnull !4, !noundef !4
  %4 = mul i64 %self.1, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %self, ptr align 1 %self.0, i64 %4, i1 false)
  %5 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %v, i32 0, i32 1
  store i64 %self.1, ptr %5, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %bytes, ptr align 8 %v, i64 24, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %bytes, i64 24, i1 false)
  ret void
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h84641e80bdcba56eE(ptr align 1 %self, i64 %0, i64 %1, i1 zeroext %zeroed) unnamed_addr #0 {
start:
  %2 = alloca i8, align 1
  %_79 = alloca %"core::ptr::metadata::PtrComponents<[u8]>", align 8
  %_78 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %_64 = alloca ptr, align 8
  %_59 = alloca i64, align 8
  %_44 = alloca i64, align 8
  %_34 = alloca %"core::ptr::metadata::PtrComponents<[u8]>", align 8
  %_33 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %_22 = alloca i64, align 8
  %_18 = alloca %"core::ptr::non_null::NonNull<[u8]>", align 8
  %self4 = alloca ptr, align 8
  %self3 = alloca ptr, align 8
  %_12 = alloca ptr, align 8
  %layout2 = alloca %"core::alloc::layout::Layout", align 8
  %layout1 = alloca %"core::alloc::layout::Layout", align 8
  %raw_ptr = alloca ptr, align 8
  %data = alloca ptr, align 8
  %_6 = alloca %"core::ptr::non_null::NonNull<[u8]>", align 8
  %_0 = alloca %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>", align 8
  %layout = alloca %"core::alloc::layout::Layout", align 8
  store i64 %0, ptr %layout, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %layout, i64 8
  %size = load i64, ptr %4, align 8, !noundef !4
  %5 = icmp eq i64 %size, 0
  br i1 %5, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %self5 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  store i64 %self5, ptr %_22, align 8
  %_23 = load i64, ptr %_22, align 8, !range !6, !noundef !4
  %_24 = icmp uge i64 %_23, 1
  %_25 = icmp ule i64 %_23, -9223372036854775808
  %_26 = and i1 %_24, %_25
  call void @llvm.assume(i1 %_26)
  %ptr = inttoptr i64 %_23 to ptr
  store ptr %ptr, ptr %data, align 8
  store ptr %ptr, ptr %_34, align 8
  %6 = getelementptr inbounds i8, ptr %_34, i64 8
  store i64 0, ptr %6, align 8
  %7 = load ptr, ptr %_34, align 8, !noundef !4
  %8 = getelementptr inbounds i8, ptr %_34, i64 8
  %9 = load i64, ptr %8, align 8, !noundef !4
  store ptr %7, ptr %_33, align 8
  %10 = getelementptr inbounds i8, ptr %_33, i64 8
  store i64 %9, ptr %10, align 8
  %ptr.0 = load ptr, ptr %_33, align 8, !noundef !4
  %11 = getelementptr inbounds i8, ptr %_33, i64 8
  %ptr.1 = load i64, ptr %11, align 8, !noundef !4
  store ptr %ptr.0, ptr %_6, align 8
  %12 = getelementptr inbounds i8, ptr %_6, i64 8
  store i64 %ptr.1, ptr %12, align 8
  %13 = load ptr, ptr %_6, align 8, !nonnull !4, !noundef !4
  %14 = getelementptr inbounds i8, ptr %_6, i64 8
  %15 = load i64, ptr %14, align 8, !noundef !4
  store ptr %13, ptr %_0, align 8
  %16 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %15, ptr %16, align 8
  br label %bb8

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb8:                                              ; preds = %bb7, %bb6, %bb2
  %17 = load ptr, ptr %_0, align 8, !noundef !4
  %18 = getelementptr inbounds i8, ptr %_0, i64 8
  %19 = load i64, ptr %18, align 8
  %20 = insertvalue { ptr, i64 } poison, ptr %17, 0
  %21 = insertvalue { ptr, i64 } %20, i64 %19, 1
  ret { ptr, i64 } %21

bb4:                                              ; preds = %bb1
  %22 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %23 = getelementptr inbounds i8, ptr %layout, i64 8
  %24 = load i64, ptr %23, align 8, !noundef !4
  store i64 %22, ptr %layout2, align 8
  %25 = getelementptr inbounds i8, ptr %layout2, i64 8
  store i64 %24, ptr %25, align 8
  %26 = load volatile i8, ptr @__rust_no_alloc_shim_is_unstable, align 1
  store i8 %26, ptr %2, align 1
  %_49 = load i8, ptr %2, align 1, !noundef !4
  %27 = getelementptr inbounds i8, ptr %layout2, i64 8
  %_52 = load i64, ptr %27, align 8, !noundef !4
  %self6 = load i64, ptr %layout2, align 8, !range !6, !noundef !4
  store i64 %self6, ptr %_59, align 8
  %_60 = load i64, ptr %_59, align 8, !range !6, !noundef !4
  %_61 = icmp uge i64 %_60, 1
  %_62 = icmp ule i64 %_60, -9223372036854775808
  %_63 = and i1 %_61, %_62
  call void @llvm.assume(i1 %_63)
  %28 = call ptr @__rust_alloc(i64 %_52, i64 %_60) #15
  store ptr %28, ptr %raw_ptr, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
  %29 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %30 = getelementptr inbounds i8, ptr %layout, i64 8
  %31 = load i64, ptr %30, align 8, !noundef !4
  store i64 %29, ptr %layout1, align 8
  %32 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %31, ptr %32, align 8
  %33 = getelementptr inbounds i8, ptr %layout1, i64 8
  %_39 = load i64, ptr %33, align 8, !noundef !4
  %self7 = load i64, ptr %layout1, align 8, !range !6, !noundef !4
  store i64 %self7, ptr %_44, align 8
  %_45 = load i64, ptr %_44, align 8, !range !6, !noundef !4
  %_46 = icmp uge i64 %_45, 1
  %_47 = icmp ule i64 %_45, -9223372036854775808
  %_48 = and i1 %_46, %_47
  call void @llvm.assume(i1 %_48)
  %34 = call ptr @__rust_alloc_zeroed(i64 %_39, i64 %_45) #15
  store ptr %34, ptr %raw_ptr, align 8
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %ptr8 = load ptr, ptr %raw_ptr, align 8, !noundef !4
  %_65 = ptrtoint ptr %ptr8 to i64
  %35 = icmp eq i64 %_65, 0
  br i1 %35, label %bb13, label %bb14

bb13:                                             ; preds = %bb5
  store ptr null, ptr %self4, align 8
  br label %bb12

bb14:                                             ; preds = %bb5
  store ptr %ptr8, ptr %_64, align 8
  %36 = load ptr, ptr %_64, align 8, !nonnull !4, !noundef !4
  store ptr %36, ptr %self4, align 8
  br label %bb12

bb12:                                             ; preds = %bb14, %bb13
  %37 = load ptr, ptr %self4, align 8, !noundef !4
  %38 = ptrtoint ptr %37 to i64
  %39 = icmp eq i64 %38, 0
  %_70 = select i1 %39, i64 0, i64 1
  %40 = icmp eq i64 %_70, 0
  br i1 %40, label %bb15, label %bb16

bb15:                                             ; preds = %bb12
  store ptr null, ptr %self3, align 8
  br label %bb17

bb16:                                             ; preds = %bb12
  %v = load ptr, ptr %self4, align 8, !nonnull !4, !noundef !4
  store ptr %v, ptr %self3, align 8
  br label %bb17

bb17:                                             ; preds = %bb16, %bb15
  %41 = load ptr, ptr %self3, align 8, !noundef !4
  %42 = ptrtoint ptr %41 to i64
  %43 = icmp eq i64 %42, 0
  %_72 = select i1 %43, i64 1, i64 0
  %44 = icmp eq i64 %_72, 0
  br i1 %44, label %bb20, label %bb19

bb20:                                             ; preds = %bb17
  %v9 = load ptr, ptr %self3, align 8, !nonnull !4, !noundef !4
  store ptr %v9, ptr %_12, align 8
  br label %bb18

bb19:                                             ; preds = %bb17
  store ptr null, ptr %_12, align 8
  br label %bb18

bb18:                                             ; preds = %bb19, %bb20
  %45 = load ptr, ptr %_12, align 8, !noundef !4
  %46 = ptrtoint ptr %45 to i64
  %47 = icmp eq i64 %46, 0
  %_16 = select i1 %47, i64 1, i64 0
  %48 = icmp eq i64 %_16, 0
  br i1 %48, label %bb6, label %bb7

bb6:                                              ; preds = %bb18
  %ptr10 = load ptr, ptr %_12, align 8, !nonnull !4, !noundef !4
  store ptr %ptr10, ptr %_79, align 8
  %49 = getelementptr inbounds i8, ptr %_79, i64 8
  store i64 %size, ptr %49, align 8
  %50 = load ptr, ptr %_79, align 8, !noundef !4
  %51 = getelementptr inbounds i8, ptr %_79, i64 8
  %52 = load i64, ptr %51, align 8, !noundef !4
  store ptr %50, ptr %_78, align 8
  %53 = getelementptr inbounds i8, ptr %_78, i64 8
  store i64 %52, ptr %53, align 8
  %ptr.011 = load ptr, ptr %_78, align 8, !noundef !4
  %54 = getelementptr inbounds i8, ptr %_78, i64 8
  %ptr.112 = load i64, ptr %54, align 8, !noundef !4
  store ptr %ptr.011, ptr %_18, align 8
  %55 = getelementptr inbounds i8, ptr %_18, i64 8
  store i64 %ptr.112, ptr %55, align 8
  %56 = load ptr, ptr %_18, align 8, !nonnull !4, !noundef !4
  %57 = getelementptr inbounds i8, ptr %_18, i64 8
  %58 = load i64, ptr %57, align 8, !noundef !4
  store ptr %56, ptr %_0, align 8
  %59 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %58, ptr %59, align 8
  br label %bb8

bb7:                                              ; preds = %bb18
  %60 = load ptr, ptr @0, align 8, !noundef !4
  %61 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %60, ptr %_0, align 8
  %62 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %61, ptr %62, align 8
  br label %bb8

bb21:                                             ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::allocate_in
; Function Attrs: nonlazybind uwtable
define { i64, ptr } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hf6de168e2ebec739E"(i64 %capacity, i1 zeroext %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32, [1 x i32] }, align 8
  %_46 = alloca ptr, align 8
  %_29 = alloca ptr, align 8
  %_28 = alloca ptr, align 8
  %_26 = alloca i64, align 8
  %self = alloca ptr, align 8
  %_23 = alloca ptr, align 8
  %result = alloca %"core::result::Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>", align 8
  %_7 = alloca %"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>", align 8
  %layout = alloca %"core::alloc::layout::Layout", align 8
  %_0 = alloca %"alloc::raw_vec::RawVec<u8>", align 8
  %alloc = alloca %"alloc::alloc::Global", align 1
  %init = alloca i8, align 1
  %2 = zext i1 %0 to i8
  store i8 %2, ptr %init, align 1
  br i1 false, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %3 = icmp eq i64 %capacity, 0
  br i1 %3, label %bb2, label %bb3

bb2:                                              ; preds = %bb1, %start
  store ptr inttoptr (i64 1 to ptr), ptr %_29, align 8
  %4 = load ptr, ptr %_29, align 8, !nonnull !4, !noundef !4
  store ptr %4, ptr %_28, align 8
  %5 = load ptr, ptr %_28, align 8, !nonnull !4, !noundef !4
  %6 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr %5, ptr %6, align 8
  store i64 0, ptr %_0, align 8
  br label %bb13

bb3:                                              ; preds = %bb1
; invoke core::alloc::layout::Layout::array::inner
  %7 = invoke { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h2f13b444b6013ad3E(i64 1, i64 1, i64 %capacity)
          to label %bb16 unwind label %cleanup

bb15:                                             ; preds = %cleanup
  %8 = load ptr, ptr %1, align 8, !noundef !4
  %9 = getelementptr inbounds i8, ptr %1, i64 8
  %10 = load i32, ptr %9, align 8, !noundef !4
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1
  resume { ptr, i32 } %12

cleanup:                                          ; preds = %bb4, %bb11, %bb6, %bb7, %bb3
  %13 = landingpad { ptr, i32 }
          cleanup
  %14 = extractvalue { ptr, i32 } %13, 0
  %15 = extractvalue { ptr, i32 } %13, 1
  store ptr %14, ptr %1, align 8
  %16 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %15, ptr %16, align 8
  br label %bb15

bb16:                                             ; preds = %bb3
  %17 = extractvalue { i64, i64 } %7, 0
  %18 = extractvalue { i64, i64 } %7, 1
  store i64 %17, ptr %_7, align 8
  %19 = getelementptr inbounds i8, ptr %_7, i64 8
  store i64 %18, ptr %19, align 8
  %20 = load i64, ptr %_7, align 8, !range !7, !noundef !4
  %21 = icmp eq i64 %20, 0
  %_8 = select i1 %21, i64 1, i64 0
  %22 = icmp eq i64 %_8, 0
  br i1 %22, label %bb5, label %bb4

bb5:                                              ; preds = %bb16
  %layout.0 = load i64, ptr %_7, align 8, !range !6, !noundef !4
  %23 = getelementptr inbounds i8, ptr %_7, i64 8
  %layout.1 = load i64, ptr %23, align 8, !noundef !4
  store i64 %layout.0, ptr %layout, align 8
  %24 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %layout.1, ptr %24, align 8
  %25 = getelementptr inbounds i8, ptr %layout, i64 8
  %alloc_size = load i64, ptr %25, align 8, !noundef !4
  %26 = load i8, ptr %init, align 1, !range !8, !noundef !4
  %27 = trunc i8 %26 to i1
  %_14 = zext i1 %27 to i64
  %28 = icmp eq i64 %_14, 0
  br i1 %28, label %bb7, label %bb6

bb4:                                              ; preds = %bb16
; invoke alloc::raw_vec::capacity_overflow
  invoke void @_ZN5alloc7raw_vec17capacity_overflow17hbca7785f3bc15d50E() #12
          to label %unreachable unwind label %cleanup

bb7:                                              ; preds = %bb5
  %_16.0 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %29 = getelementptr inbounds i8, ptr %layout, i64 8
  %_16.1 = load i64, ptr %29, align 8, !noundef !4
; invoke <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %30 = invoke { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he7a8b480edfee5e1E"(ptr align 1 %alloc, i64 %_16.0, i64 %_16.1)
          to label %bb8 unwind label %cleanup

bb6:                                              ; preds = %bb5
  %_18.0 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %31 = getelementptr inbounds i8, ptr %layout, i64 8
  %_18.1 = load i64, ptr %31, align 8, !noundef !4
; invoke <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
  %32 = invoke { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h2ec0357c30f6eb0fE"(ptr align 1 %alloc, i64 %_18.0, i64 %_18.1)
          to label %bb9 unwind label %cleanup

bb8:                                              ; preds = %bb7
  %33 = extractvalue { ptr, i64 } %30, 0
  %34 = extractvalue { ptr, i64 } %30, 1
  store ptr %33, ptr %result, align 8
  %35 = getelementptr inbounds i8, ptr %result, i64 8
  store i64 %34, ptr %35, align 8
  br label %bb10

bb10:                                             ; preds = %bb9, %bb8
  %36 = load ptr, ptr %result, align 8, !noundef !4
  %37 = ptrtoint ptr %36 to i64
  %38 = icmp eq i64 %37, 0
  %_19 = select i1 %38, i64 1, i64 0
  %39 = icmp eq i64 %_19, 0
  br i1 %39, label %bb12, label %bb11

bb9:                                              ; preds = %bb6
  %40 = extractvalue { ptr, i64 } %32, 0
  %41 = extractvalue { ptr, i64 } %32, 1
  store ptr %40, ptr %result, align 8
  %42 = getelementptr inbounds i8, ptr %result, i64 8
  store i64 %41, ptr %42, align 8
  br label %bb10

bb12:                                             ; preds = %bb10
  %ptr.0 = load ptr, ptr %result, align 8, !nonnull !4, !noundef !4
  %43 = getelementptr inbounds i8, ptr %result, i64 8
  %ptr.1 = load i64, ptr %43, align 8, !noundef !4
  store ptr %ptr.0, ptr %self, align 8
  store ptr %ptr.0, ptr %_46, align 8
  %44 = load ptr, ptr %_46, align 8, !nonnull !4, !noundef !4
  store ptr %44, ptr %_23, align 8
  store i64 %capacity, ptr %_26, align 8
  %45 = load ptr, ptr %_23, align 8, !nonnull !4, !noundef !4
  %46 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr %45, ptr %46, align 8
  %47 = load i64, ptr %_26, align 8, !range !9, !noundef !4
  store i64 %47, ptr %_0, align 8
  br label %bb13

bb11:                                             ; preds = %bb10
  %_22.0 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %48 = getelementptr inbounds i8, ptr %layout, i64 8
  %_22.1 = load i64, ptr %48, align 8, !noundef !4
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h426354a964e0805cE(i64 %_22.0, i64 %_22.1) #12
          to label %unreachable unwind label %cleanup

bb13:                                             ; preds = %bb2, %bb12
  %49 = load i64, ptr %_0, align 8, !range !9, !noundef !4
  %50 = getelementptr inbounds i8, ptr %_0, i64 8
  %51 = load ptr, ptr %50, align 8, !nonnull !4, !noundef !4
  %52 = insertvalue { i64, ptr } poison, i64 %49, 0
  %53 = insertvalue { i64, ptr } %52, ptr %51, 1
  ret { i64, ptr } %53

unreachable:                                      ; preds = %bb4, %bb11
  unreachable

bb17:                                             ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: nonlazybind uwtable
define void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hc9a6e325e2fafc35E"(ptr sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") align 8 %_0, ptr align 8 %self) unnamed_addr #1 {
start:
  %self2 = alloca ptr, align 8
  %self1 = alloca ptr, align 8
  %_10 = alloca ptr, align 8
  %_9 = alloca { ptr, %"core::alloc::layout::Layout" }, align 8
  %layout = alloca %"core::alloc::layout::Layout", align 8
  br i1 false, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %_3 = load i64, ptr %self, align 8, !noundef !4
  %0 = icmp eq i64 %_3, 0
  br i1 %0, label %bb2, label %bb3

bb2:                                              ; preds = %bb1, %start
  %1 = getelementptr inbounds %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", ptr %_0, i32 0, i32 1
  store i64 0, ptr %1, align 8
  br label %bb4

bb3:                                              ; preds = %bb1
  %rhs = load i64, ptr %self, align 8, !noundef !4
  %size = mul nuw i64 1, %rhs
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %size, ptr %2, align 8
  store i64 1, ptr %layout, align 8
  %3 = getelementptr inbounds i8, ptr %self, i64 8
  %self3 = load ptr, ptr %3, align 8, !nonnull !4, !noundef !4
  store ptr %self3, ptr %self1, align 8
  store ptr %self3, ptr %self2, align 8
  store ptr %self3, ptr %_10, align 8
  %4 = load ptr, ptr %_10, align 8, !nonnull !4, !noundef !4
  store ptr %4, ptr %_9, align 8
  %5 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %6 = getelementptr inbounds i8, ptr %layout, i64 8
  %7 = load i64, ptr %6, align 8, !noundef !4
  %8 = getelementptr inbounds { ptr, %"core::alloc::layout::Layout" }, ptr %_9, i32 0, i32 1
  store i64 %5, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 %7, ptr %9, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_9, i64 24, i1 false)
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  ret void
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h8d5cf114f69db4f4E"(ptr align 1 %self, ptr %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %_14 = alloca i64, align 8
  %layout1 = alloca %"core::alloc::layout::Layout", align 8
  %layout = alloca %"core::alloc::layout::Layout", align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %_4 = load i64, ptr %3, align 8, !noundef !4
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %start
  br label %bb3

bb1:                                              ; preds = %start
  %5 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %6 = getelementptr inbounds i8, ptr %layout, i64 8
  %7 = load i64, ptr %6, align 8, !noundef !4
  store i64 %5, ptr %layout1, align 8
  %8 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %7, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %layout1, i64 8
  %_9 = load i64, ptr %9, align 8, !noundef !4
  %self2 = load i64, ptr %layout1, align 8, !range !6, !noundef !4
  store i64 %self2, ptr %_14, align 8
  %_15 = load i64, ptr %_14, align 8, !range !6, !noundef !4
  %_16 = icmp uge i64 %_15, 1
  %_17 = icmp ule i64 %_15, -9223372036854775808
  %_18 = and i1 %_16, %_17
  call void @llvm.assume(i1 %_18)
  call void @__rust_dealloc(ptr %ptr, i64 %_9, i64 %_15) #15
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  ret void
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h2ec0357c30f6eb0fE"(ptr align 1 %self, i64 %layout.0, i64 %layout.1) unnamed_addr #0 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h84641e80bdcba56eE(ptr align 1 %self, i64 %layout.0, i64 %layout.1, i1 zeroext true)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he7a8b480edfee5e1E"(ptr align 1 %self, i64 %layout.0, i64 %layout.1) unnamed_addr #0 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h84641e80bdcba56eE(ptr align 1 %self, i64 %layout.0, i64 %layout.1, i1 zeroext false)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h54cf3ba6880e5fa7E"(ptr align 8 %self) unnamed_addr #1 {
start:
  %_10 = alloca %"core::ptr::metadata::PtrComponents<[u8]>", align 8
  %_9 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %self, i32 0, i32 1
  %len = load i64, ptr %1, align 8, !noundef !4
  store ptr %self1, ptr %_10, align 8
  %2 = getelementptr inbounds i8, ptr %_10, i64 8
  store i64 %len, ptr %2, align 8
  %3 = load ptr, ptr %_10, align 8, !noundef !4
  %4 = getelementptr inbounds i8, ptr %_10, i64 8
  %5 = load i64, ptr %4, align 8, !noundef !4
  store ptr %3, ptr %_9, align 8
  %6 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 %5, ptr %6, align 8
  %_2.0 = load ptr, ptr %_9, align 8, !noundef !4
  %7 = getelementptr inbounds i8, ptr %_9, i64 8
  %_2.1 = load i64, ptr %7, align 8, !noundef !4
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h076e862acfb0ec91E"(ptr align 8 %self) unnamed_addr #1 {
start:
  %_2 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 8
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hc9a6e325e2fafc35E"(ptr sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") align 8 %_2, ptr align 8 %self)
  %0 = getelementptr inbounds %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", ptr %_2, i32 0, i32 1
  %1 = load i64, ptr %0, align 8, !range !7, !noundef !4
  %2 = icmp eq i64 %1, 0
  %_4 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_4, 1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %start
  %ptr = load ptr, ptr %_2, align 8, !nonnull !4, !noundef !4
  %4 = getelementptr inbounds { ptr, %"core::alloc::layout::Layout" }, ptr %_2, i32 0, i32 1
  %layout.0 = load i64, ptr %4, align 8, !range !6, !noundef !4
  %5 = getelementptr inbounds i8, ptr %4, i64 8
  %layout.1 = load i64, ptr %5, align 8, !noundef !4
  %_7 = getelementptr inbounds i8, ptr %self, i64 16
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h8d5cf114f69db4f4E"(ptr align 1 %_7, ptr %ptr, i64 %layout.0, i64 %layout.1)
  br label %bb4

bb4:                                              ; preds = %bb2, %start
  ret void
}

; probe1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe15probe17h1d147e5977d51852E() unnamed_addr #1 {
start:
  %_0.i = alloca %"core::fmt::rt::Argument<'_>", align 8
  %_7 = alloca [1 x %"core::fmt::rt::Argument<'_>"], align 8
  %_3 = alloca %"core::fmt::Arguments<'_>", align 8
  %res = alloca %"alloc::string::String", align 8
  %_1 = alloca %"alloc::string::String", align 8
  store ptr @alloc_53973d2fe29b4adba8bb7390b5678745, ptr %_0.i, align 8
  %0 = getelementptr inbounds i8, ptr %_0.i, i64 8
  store ptr @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17h316887caab73c865E", ptr %0, align 8
  %1 = load ptr, ptr %_0.i, align 8, !nonnull !4, !align !5, !noundef !4
  %2 = getelementptr inbounds i8, ptr %_0.i, i64 8
  %3 = load ptr, ptr %2, align 8, !nonnull !4, !noundef !4
  %4 = insertvalue { ptr, ptr } poison, ptr %1, 0
  %5 = insertvalue { ptr, ptr } %4, ptr %3, 1
  %_8.0 = extractvalue { ptr, ptr } %5, 0
  %_8.1 = extractvalue { ptr, ptr } %5, 1
  %6 = getelementptr inbounds [1 x %"core::fmt::rt::Argument<'_>"], ptr %_7, i64 0, i64 0
  store ptr %_8.0, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store ptr %_8.1, ptr %7, align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h82784979634012efE(ptr sret(%"core::fmt::Arguments<'_>") align 8 %_3, ptr align 8 @alloc_b99730e73100e73a81f4fbfe74b3821d, i64 1, ptr align 8 %_7, i64 1)
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17h05bcfe910207afbfE(ptr sret(%"alloc::string::String") align 8 %res, ptr align 8 %_3)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_1, ptr align 8 %res, i64 24, i1 false)
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4ca7eb99b039b66aE"(ptr align 8 %_1)
  ret void
}

; core::fmt::num::imp::<impl core::fmt::LowerExp for isize>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17h316887caab73c865E"(ptr align 8, ptr align 8) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17ha6effc2775a0749cE(ptr align 8, ptr align 8) unnamed_addr #2

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; core::panicking::panic_in_cleanup
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17hbacfddf1bcf21a1eE() unnamed_addr #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.assume(i1 noundef) #4

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #5

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h44790a89027c670fE(ptr align 1, i64, ptr align 8) unnamed_addr #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #6

; alloc::fmt::format::format_inner
; Function Attrs: nonlazybind uwtable
declare void @_ZN5alloc3fmt6format12format_inner17h20bbaee2ca87fbecE(ptr sret(%"alloc::string::String") align 8, ptr align 8) unnamed_addr #1

; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc(i64, i64 allocalign) unnamed_addr #7

; Function Attrs: nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc_zeroed(i64, i64 allocalign) unnamed_addr #8

; alloc::alloc::handle_alloc_error
; Function Attrs: cold noreturn nonlazybind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h426354a964e0805cE(i64, i64) unnamed_addr #9

; alloc::raw_vec::capacity_overflow
; Function Attrs: noinline noreturn nonlazybind uwtable
declare void @_ZN5alloc7raw_vec17capacity_overflow17hbca7785f3bc15d50E() unnamed_addr #10

; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr, i64, i64) unnamed_addr #11

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #5 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #6 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #8 = { nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #9 = { cold noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #10 = { noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #11 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #12 = { noreturn }
attributes #13 = { cold }
attributes #14 = { cold noreturn nounwind }
attributes #15 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.77.2 (25ef9e3d8 2024-04-09)"}
!3 = !{i64 8}
!4 = !{}
!5 = !{i64 1}
!6 = !{i64 1, i64 -9223372036854775807}
!7 = !{i64 0, i64 -9223372036854775807}
!8 = !{i8 0, i8 2}
!9 = !{i64 0, i64 -9223372036854775808}
