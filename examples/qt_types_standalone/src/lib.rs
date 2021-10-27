// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::pin::Pin;
use cxx_qt_lib::{
    let_qcolor, let_qstring, let_qvariant, Color, MapQtValue, QColor, QPointF, QSizeF, QString,
    QVariant, Variant, VariantImpl,
};

#[cxx::bridge]
mod ffi {
    enum ColorTest {
        Rgb_Red,
        Rgb_Green,
        Rgb_Blue,
        Rgb_Transparent,
    }

    enum VariantTest {
        String,
        Int,
        Bool,
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/statics/rust/cxx_qt.h");
        include!("bridge.h");

        type QColor = cxx_qt_lib::QColor;
        type QString = cxx_qt_lib::QString;
        type QVariant = cxx_qt_lib::QVariant;
        type QSizeF = cxx_qt_lib::QSizeF;
        type QPointF = cxx_qt_lib::QPointF;

        #[namespace = "CxxQt"]
        type Variant = cxx_qt_lib::Variant;

        fn test_constructed_qstring(s: &QString) -> bool;
        fn assign_to_qstring(s: Pin<&mut QString>, v: &QString);

        fn test_constructed_qcolor(c: &QColor, test: ColorTest) -> bool;

        fn test_constructed_qvariant(s: &QVariant, test: VariantTest) -> bool;
    }

    extern "Rust" {
        fn can_construct_qstring(slice: bool) -> bool;
        fn can_read_qstring(s: &QString) -> bool;
        fn modify_qstring(s: Pin<&mut QString>);
        fn can_map_to_qstring() -> bool;
        fn can_handle_qstring_change() -> bool;

        fn can_construct_qcolor(test: ColorTest) -> bool;
        fn can_read_qcolor(c: &QColor, test: ColorTest) -> bool;

        fn make_variant(test: VariantTest) -> Variant;
        fn can_construct_qvariant(test: VariantTest) -> bool;
        fn can_read_qvariant(v: &QVariant, test: VariantTest) -> bool;

        fn construct_qpointf() -> QPointF;
        fn read_qpointf(p: &QPointF) -> bool;
        fn copy_qpointf(p: &QPointF) -> QPointF;
        fn copy_value_qpointf(p: QPointF) -> QPointF;

        fn construct_qsizef() -> QSizeF;
        fn read_qsizef(p: &QSizeF) -> bool;
        fn copy_qsizef(p: &QSizeF) -> QSizeF;
        fn copy_value_qsizef(p: QSizeF) -> QSizeF;
    }
}

use ffi::ColorTest;
use ffi::VariantTest;

fn can_construct_qstring(slice: bool) -> bool {
    if slice {
        let_qstring!(s = "String constructed by Rust");
        ffi::test_constructed_qstring(&s)
    } else {
        let rs_string = "String constructed by Rust".to_owned();
        let_qstring!(s = rs_string);
        ffi::test_constructed_qstring(&s)
    }
}

fn can_read_qstring(s: &QString) -> bool {
    let rs = s.to_rust();
    rs == "String constructed by C++"
}

fn modify_qstring(s: Pin<&mut QString>) {
    let_qstring!(v = "Updated string value");
    ffi::assign_to_qstring(s, &v);
}

fn can_map_to_qstring() -> bool {
    "String constructed by Rust".map_qt_value(
        |_, converted| ffi::test_constructed_qstring(converted),
        &mut (),
    )
}

fn can_handle_qstring_change() -> bool {
    let long_s = "Very very long string that is hopefully long enough to allocate and get Valgrind's attention :)";

    let_qstring!(s = "Short string");
    let_qstring!(v = long_s);
    ffi::assign_to_qstring(s.as_mut(), &v);

    let rs = s.to_rust();
    rs == long_s
}

fn can_construct_qcolor(test: ColorTest) -> bool {
    let color = match test {
        ColorTest::Rgb_Red => Color::ARGB {
            alpha: 255,
            red: 255,
            green: 0,
            blue: 0,
        },
        ColorTest::Rgb_Green => Color::ARGB {
            alpha: 255,
            red: 0,
            green: 255,
            blue: 0,
        },
        ColorTest::Rgb_Blue => Color::ARGB {
            alpha: 255,
            red: 0,
            green: 0,
            blue: 255,
        },
        ColorTest::Rgb_Transparent => Color::ARGB {
            alpha: 0,
            red: 0,
            green: 0,
            blue: 0,
        },
        _others => panic!("Unsupported test: {}", test.repr),
    };

    let_qcolor!(c = &color);
    ffi::test_constructed_qcolor(&c, test)
}

fn can_read_qcolor(c: &QColor, test: ColorTest) -> bool {
    match test {
        ColorTest::Rgb_Red => {
            let rs_c = c.to_rust();
            match rs_c {
                Some(Color::ARGB {
                    alpha,
                    red,
                    green,
                    blue,
                }) => alpha == 255 && red == 255 && green == 0 && blue == 0,
                _others => false,
            }
        }
        ColorTest::Rgb_Green => {
            let rs_c = c.to_rust();
            match rs_c {
                Some(Color::ARGB {
                    alpha,
                    red,
                    green,
                    blue,
                }) => alpha == 255 && red == 0 && green == 255 && blue == 0,
                _others => false,
            }
        }
        ColorTest::Rgb_Blue => {
            let rs_c = c.to_rust();
            match rs_c {
                Some(Color::ARGB {
                    alpha,
                    red,
                    green,
                    blue,
                }) => alpha == 255 && red == 0 && green == 0 && blue == 255,
                _others => false,
            }
        }
        ColorTest::Rgb_Transparent => {
            let rs_c = c.to_rust();
            match rs_c {
                Some(Color::ARGB {
                    alpha,
                    red,
                    green,
                    blue,
                }) => alpha == 0 && red == 0 && green == 0 && blue == 0,
                _others => false,
            }
        }
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn make_variant(test: VariantTest) -> Variant {
    match test {
        VariantTest::String => Variant::from_string("Rust string".to_owned()),
        VariantTest::Int => Variant::from_int(123),
        VariantTest::Bool => Variant::from_bool(true),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn can_construct_qvariant(test: VariantTest) -> bool {
    let variant = make_variant(test);
    let_qvariant!(v = &variant);
    ffi::test_constructed_qvariant(&v, test)
}

fn can_read_qvariant(v: &QVariant, test: VariantTest) -> bool {
    match test {
        VariantTest::String => match v.to_rust().as_deref() {
            Some(VariantImpl::String(s)) => s == "C++ string",
            _others => false,
        },
        VariantTest::Int => match v.to_rust().as_deref() {
            Some(VariantImpl::Int(i)) => *i == 8910,
            _others => false,
        },
        VariantTest::Bool => match v.to_rust().as_deref() {
            Some(VariantImpl::Bool(b)) => !*b,
            _others => false,
        },
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn construct_qpointf() -> QPointF {
    QPointF::new(1.23, 4.56)
}

fn read_qpointf(p: &QPointF) -> bool {
    ((p.x() - 1.23).abs() < f64::EPSILON) && ((p.y() - 4.56).abs() < f64::EPSILON)
}

fn copy_qpointf(p: &QPointF) -> QPointF {
    *p
}

fn copy_value_qpointf(p: QPointF) -> QPointF {
    p
}

fn construct_qsizef() -> QSizeF {
    QSizeF::new(1.23, 4.56)
}

fn read_qsizef(s: &QSizeF) -> bool {
    ((s.w - 1.23).abs() < f64::EPSILON) && ((s.h - 4.56).abs() < f64::EPSILON)
}

fn copy_qsizef(s: &QSizeF) -> QSizeF {
    *s
}

fn copy_value_qsizef(s: QSizeF) -> QSizeF {
    s
}