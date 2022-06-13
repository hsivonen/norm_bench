// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use icu_normalizer::DecomposingNormalizer;
use icu_normalizer::ComposingNormalizer;
use rust_icu_ustring::UChar;
use rust_icu_unorm2::UNormalizer;
use unicode_normalization::UnicodeNormalization;
use unic_normal::StrNormalForm;
use detone::IterDecomposeVietnamese;

fn slice_from_icu4c(string: &UChar) -> &[u16] {
    // Can't find this on `UChar` itself.
    unsafe { core::slice::from_raw_parts(string.as_c_ptr(), string.len()) }
}

fn bench_lang(name: &str, data: &str, c: &mut Criterion) {
    let data_provider = icu_testdata::get_provider();

    let nfd_norm_icu4x = DecomposingNormalizer::try_new_nfd(&data_provider).unwrap();
    let nfkd_norm_icu4x = DecomposingNormalizer::try_new_nfkd(&data_provider).unwrap();
    let nfc_norm_icu4x = ComposingNormalizer::try_new_nfc(&data_provider).unwrap();
    let nfkc_norm_icu4x = ComposingNormalizer::try_new_nfkc(&data_provider).unwrap();

    let nfd_icu4x = nfd_norm_icu4x.normalize(data);
    let nfkd_icu4x = nfkd_norm_icu4x.normalize(data);
    let nfc_icu4x = nfc_norm_icu4x.normalize(data);
    let nfkc_icu4x = nfkc_norm_icu4x.normalize(data);

    let nfd_norm_icu4c = UNormalizer::new_nfd().unwrap();
    let nfkd_norm_icu4c = UNormalizer::new_nfkd().unwrap();
    let nfc_norm_icu4c = UNormalizer::new_nfc().unwrap();
    let nfkc_norm_icu4c = UNormalizer::new_nfkc().unwrap();

    let mut data_utf16 = Vec::new();
    let mut buf = [0; 2];
    for c in data.chars() {
        let enc = c.encode_utf16(&mut buf);
        data_utf16.extend_from_slice(enc);
    }

    let nfd_utf16_icu4x = nfd_norm_icu4x.normalize_utf16(&data_utf16);
    let nfkd_utf16_icu4x = nfkd_norm_icu4x.normalize_utf16(&data_utf16);
    let nfc_utf16_icu4x = nfc_norm_icu4x.normalize_utf16(&data_utf16);
    let nfkc_utf16_icu4x = nfkc_norm_icu4x.normalize_utf16(&data_utf16);

    let data_uchar: UChar = data_utf16.clone().into();

    let nfd_utf16_icu4c = nfd_norm_icu4c.normalize_ustring(&data_uchar).unwrap();
    let nfkd_utf16_icu4c = nfkd_norm_icu4c.normalize_ustring(&data_uchar).unwrap();
    let nfc_utf16_icu4c = nfc_norm_icu4c.normalize_ustring(&data_uchar).unwrap();
    let nfkc_utf16_icu4c = nfkc_norm_icu4c.normalize_ustring(&data_uchar).unwrap();

    assert_eq!(&nfd_utf16_icu4x[..], slice_from_icu4c(&nfd_utf16_icu4c));
    assert_eq!(&nfkd_utf16_icu4x[..], slice_from_icu4c(&nfkd_utf16_icu4c));
    assert_eq!(&nfc_utf16_icu4x[..], slice_from_icu4c(&nfc_utf16_icu4c));
    assert_eq!(&nfkc_utf16_icu4x[..], slice_from_icu4c(&nfkc_utf16_icu4c));

    let nfd_unic = StrNormalForm::nfd(data).collect::<String>();
    let nfkd_unic = StrNormalForm::nfkd(data).collect::<String>();
    let nfc_unic = StrNormalForm::nfc(data).collect::<String>();
    let nfkc_unic = StrNormalForm::nfkc(data).collect::<String>();

    assert_eq!(nfd_icu4x, nfd_unic);
    assert_eq!(nfkd_icu4x, nfkd_unic);
    assert_eq!(nfc_icu4x, nfc_unic);
    assert_eq!(nfkc_icu4x, nfkc_unic);

    let nfd_rs = UnicodeNormalization::nfd(data).collect::<String>();
    let nfkd_rs = UnicodeNormalization::nfkd(data).collect::<String>();
    let nfc_rs = UnicodeNormalization::nfc(data).collect::<String>();
    let nfkc_rs = UnicodeNormalization::nfkc(data).collect::<String>();

    assert_eq!(nfd_icu4x, nfd_rs);
    assert_eq!(nfkd_icu4x, nfkd_rs);
    assert_eq!(nfc_icu4x, nfc_rs);
    assert_eq!(nfkc_icu4x, nfkc_rs);

    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfc_to_nfc_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfc_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfc_norm_icu4x.normalize(black_box(nfc_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfc(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfc(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfc_to_nfd_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfc_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfd_norm_icu4x.normalize(black_box(nfc_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfd(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfd(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfd_to_nfd_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfd_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfd_norm_icu4x.normalize(black_box(nfd_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfd(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfd(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfd_to_nfc_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfd_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfc_norm_icu4x.normalize(black_box(nfd_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfc(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfc(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfd_to_nfkc_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfd_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfkc_norm_icu4x.normalize(black_box(nfd_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkc(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkc(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfd_to_nfkd_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfd_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfkd_norm_icu4x.normalize(black_box(nfd_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkd(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkd(black_box(nfd_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfc_to_nfkc_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfc_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfkc_norm_icu4x.normalize(black_box(nfc_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkc(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkc(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfc_to_nfkd_str");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Bytes(nfc_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfkd_norm_icu4x.normalize(black_box(nfc_icu4x.as_str()));
            black_box(normalized);
        }));
        group.bench_function("unic", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkd(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.bench_function("rs", |b| b.iter(|| {
            let normalized = StrNormalForm::nfkd(black_box(nfc_icu4x.as_str())).collect::<String>();
            black_box(normalized);
        }));
        group.finish();
    }

    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfc_to_nfc_utf16");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Elements(nfc_utf16_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfc_norm_icu4x.normalize_utf16(black_box(nfc_utf16_icu4x.as_slice()));
            black_box(normalized);
        }));
        group.bench_function("icu4c", |b| b.iter(|| {
            let normalized = nfc_norm_icu4c.normalize_ustring(black_box(&nfc_utf16_icu4c)).unwrap();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfc_to_nfd_utf16");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Elements(nfc_utf16_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfd_norm_icu4x.normalize_utf16(black_box(nfc_utf16_icu4x.as_slice()));
            black_box(normalized);
        }));
        group.bench_function("icu4c", |b| b.iter(|| {
            let normalized = nfd_norm_icu4c.normalize_ustring(black_box(&nfc_utf16_icu4c)).unwrap();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfd_to_nfd_utf16");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Elements(nfd_utf16_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfd_norm_icu4x.normalize_utf16(black_box(nfd_utf16_icu4x.as_slice()));
            black_box(normalized);
        }));
        group.bench_function("icu4c", |b| b.iter(|| {
            let normalized = nfd_norm_icu4c.normalize_ustring(black_box(&nfd_utf16_icu4c)).unwrap();
            black_box(normalized);
        }));
        group.finish();
    }
    {
        let mut group_name = name.to_string();
        group_name.push_str("_nfd_to_nfc_utf16");

        let mut group = c.benchmark_group(&group_name);
        group.throughput(Throughput::Elements(nfd_utf16_icu4x.len() as u64));
        group.bench_function("icu4x", |b| b.iter(|| {
            let normalized = nfc_norm_icu4x.normalize_utf16(black_box(nfd_utf16_icu4x.as_slice()));
            black_box(normalized);
        }));
        group.bench_function("icu4c", |b| b.iter(|| {
            let normalized = nfc_norm_icu4c.normalize_ustring(black_box(&nfd_utf16_icu4c)).unwrap();
            black_box(normalized);
        }));
        group.finish();
    }

    if name == "vi" {
        let orthographic = nfc_icu4x.chars().decompose_vietnamese_tones(true).collect::<String>();
        let mut orthographic_utf16 = Vec::new();
        for c in orthographic.chars() {
            let enc = c.encode_utf16(&mut buf);
            orthographic_utf16.extend_from_slice(enc);
        }

        let orthographic_uchar: UChar = orthographic_utf16.clone().into();

        {
            let mut group = c.benchmark_group("vi_orthographic_to_nfc_str");
            group.throughput(Throughput::Bytes(orthographic.len() as u64));
            group.bench_function("icu4x", |b| b.iter(|| {
                let normalized = nfc_norm_icu4x.normalize(black_box(orthographic.as_str()));
                black_box(normalized);
            }));
            group.bench_function("unic", |b| b.iter(|| {
                let normalized = StrNormalForm::nfc(black_box(orthographic.as_str())).collect::<String>();
                black_box(normalized);
            }));
            group.bench_function("rs", |b| b.iter(|| {
                let normalized = StrNormalForm::nfc(black_box(orthographic.as_str())).collect::<String>();
                black_box(normalized);
            }));
            group.finish();
        }
        {
            let mut group = c.benchmark_group("vi_orthographic_to_nfd_str");
            group.throughput(Throughput::Bytes(orthographic.len() as u64));
            group.bench_function("icu4x", |b| b.iter(|| {
                let normalized = nfd_norm_icu4x.normalize(black_box(orthographic.as_str()));
                black_box(normalized);
            }));
            group.bench_function("unic", |b| b.iter(|| {
                let normalized = StrNormalForm::nfd(black_box(orthographic.as_str())).collect::<String>();
                black_box(normalized);
            }));
            group.bench_function("rs", |b| b.iter(|| {
                let normalized = StrNormalForm::nfd(black_box(orthographic.as_str())).collect::<String>();
                black_box(normalized);
            }));
            group.finish();
        }

        {
            let mut group = c.benchmark_group("vi_orthographic_to_nfc_utf16");
            group.throughput(Throughput::Elements(orthographic_utf16.len() as u64));
            group.bench_function("icu4x", |b| b.iter(|| {
                let normalized = nfc_norm_icu4x.normalize_utf16(black_box(orthographic_utf16.as_slice()));
                black_box(normalized);
            }));
            group.bench_function("icu4c", |b| b.iter(|| {
                let normalized = nfc_norm_icu4c.normalize_ustring(black_box(&orthographic_uchar)).unwrap();
                black_box(normalized);
            }));
            group.finish();
        }
        {
            let mut group = c.benchmark_group("vi_orthographic_to_nfd_utf16");
            group.throughput(Throughput::Elements(orthographic_utf16.len() as u64));
            group.bench_function("icu4x", |b| b.iter(|| {
                let normalized = nfd_norm_icu4x.normalize_utf16(black_box(orthographic_utf16.as_slice()));
                black_box(normalized);
            }));
            group.bench_function("icu4c", |b| b.iter(|| {
                let normalized = nfd_norm_icu4c.normalize_ustring(black_box(&orthographic_uchar)).unwrap();
                black_box(normalized);
            }));
            group.finish();
        }
    }
}

static EL: &str = include_str!("../testdata/wikipedia/el.txt");
static EN: &str = include_str!("../testdata/wikipedia/en.txt");
static FR: &str = include_str!("../testdata/wikipedia/fr.txt");
static JA: &str = include_str!("../testdata/wikipedia/ja.txt");
static KN: &str = include_str!("../testdata/wikipedia/kn.txt");
static KO: &str = include_str!("../testdata/wikipedia/ko.txt");
static VI: &str = include_str!("../testdata/wikipedia/vi.txt");
static ZH: &str = include_str!("../testdata/wikipedia/zh.txt");

fn criterion_benchmark(c: &mut Criterion) {
    bench_lang("el", EL, c);
    bench_lang("en", EN, c);
    bench_lang("fr", FR, c);
    bench_lang("ja", JA, c);
    bench_lang("kn", KN, c);
    bench_lang("ko", KO, c);
    bench_lang("vi", VI, c);
    bench_lang("zh", ZH, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
