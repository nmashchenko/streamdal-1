extern crate test;

use test::Bencher;

use protos::sp_pipeline::PipelineDataFormat::PIPELINE_DATA_FORMAT_PLAINTEXT;
use protos::sp_steps_detective::DetectiveType;
use protos::sp_steps_detective::DetectiveTypePIIKeywordMode::DETECTIVE_TYPE_PII_KEYWORD_MODE_PERFORMANCE;

use crate::detective::{parse_field, Request};
use crate::test_utils::generate_request_for_bench;

#[bench]
fn bench_ipv4_address(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_IPV4_ADDRESS,
        "object.ipv4_address",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::ip_address(&request, field);
    });
}

#[bench]
fn bench_ipv6_address(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_IPV6_ADDRESS,
        "object.ipv6_address",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::ip_address(&request, field);
    });
}

#[bench]
fn bench_has_field(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_HAS_FIELD,
        "object.ipv4_address",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::has_field(&request, field);
    });
}

#[bench]
fn bench_string_contains_all(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_STRING_CONTAINS_ALL,
        "object.ipv4_address",
        vec!["127".to_string(), "0".to_string()],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::string_contains_all(&request, field);
    });
}

#[bench]
fn bench_string_contains_any(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_STRING_CONTAINS_ANY,
        "object.ipv4_address",
        vec!["123".to_string(), "0".to_string()],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::string_contains_any(&request, field);
    });
}

#[bench]
fn bench_uuid(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_UUID,
        "object.uuid_colon",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::uuid(&request, field);
    });
}

#[bench]
fn bench_mac_address(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_MAC_ADDRESS,
        "object.mac_address",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::mac_address(&request, field);
    });
}

#[bench]
fn bench_semver(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_SEMVER,
        "object.semver",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::semver(&request, field);
    });
}

#[bench]
fn bench_hostname(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_HOSTNAME,
        "object.valid_hostname",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_core::hostname(&request, field);
    });
}

#[bench]
fn bench_email(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_PII_EMAIL,
        "object.email_plain_valid",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_pii::email(&request, field);
    });
}

#[bench]
fn bench_email_utf8(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_PII_EMAIL,
        "object.email_unicode_domain_valid",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_pii::email(&request, field);
    });
}

#[bench]
fn bench_email_payload(b: &mut Bencher) {
    let request = generate_request_for_bench(DetectiveType::DETECTIVE_TYPE_PII_EMAIL, "", vec![]);

    b.iter(|| {
        let _ = crate::detective::Detective::new().matches(&request);
    });
}

#[bench]
fn bench_credit_card(b: &mut Bencher) {
    let request = generate_request_for_bench(
        DetectiveType::DETECTIVE_TYPE_PII_CREDIT_CARD,
        "object.credit_card.visa.valid",
        vec![],
    );

    b.iter(|| {
        let field = parse_field(request.data, &request.path).unwrap();
        let _ = crate::matcher_pii_payments::credit_card(&request, field);
    });
}

#[bench]
fn bench_credit_card_payload(b: &mut Bencher) {
    let request =
        generate_request_for_bench(DetectiveType::DETECTIVE_TYPE_PII_CREDIT_CARD, "", vec![]);

    b.iter(|| {
        let _ = crate::detective::Detective::new().matches(&request);
    });
}

#[bench]
fn bench_plaintext(b: &mut Bencher) {
    let sample_text = "Hello my name is Mark, my email is mark@streamdal.com and the vin of my car is 4T1G11AKXRU906563. I have AA000000B as my NHS number. My credit card is 4111111111111111.";

    let req = &Request {
        match_type: DetectiveType::DETECTIVE_TYPE_PII_ANY,
        data: &&sample_text.as_bytes().to_vec(),
        path: "".to_string(),
        args: Vec::new(),
        negate: false,
        mode: DETECTIVE_TYPE_PII_KEYWORD_MODE_PERFORMANCE,
        data_format: PIPELINE_DATA_FORMAT_PLAINTEXT,
    };

    b.iter(|| {

        let _ =  crate::detective::Detective::new().matches_plaintext(&req);
    });
}

#[bench]
fn bench_plaintext_without_embedded(b: &mut Bencher) {
    let sample_text = std::fs::read_to_string("./assets/test-payloads/escaped_json_logs.txt").unwrap();

    // Replace { and } inside text to avoid embedded json
    let sample_text = sample_text.replace("{", "").replace("}", "");

    let req = &Request {
        match_type: DetectiveType::DETECTIVE_TYPE_PII_ANY,
        data: &&sample_text.as_bytes().to_vec(),
        path: "".to_string(),
        args: Vec::new(),
        negate: false,
        mode: DETECTIVE_TYPE_PII_KEYWORD_MODE_PERFORMANCE,
        data_format: PIPELINE_DATA_FORMAT_PLAINTEXT,
    };

    let det = crate::detective::Detective::new();

    b.iter(|| {
        let _= det.matches_plaintext(&req);
    });
}

#[bench]
fn bench_plaintext_with_embedded(b: &mut Bencher) {
    let sample_text = std::fs::read_to_string("./assets/test-payloads/escaped_json_logs.txt").unwrap();

    let req = &Request {
        match_type: DetectiveType::DETECTIVE_TYPE_PII_ANY,
        data: &&sample_text.as_bytes().to_vec(),
        path: "".to_string(),
        args: Vec::new(),
        negate: false,
        mode: DETECTIVE_TYPE_PII_KEYWORD_MODE_PERFORMANCE,
        data_format: PIPELINE_DATA_FORMAT_PLAINTEXT,
    };

    let det = crate::detective::Detective::new();

    b.iter(|| {
        let _= det.matches_plaintext(&req);
    });
}


#[bench]
fn bench_get_json_payloads(b : &mut Bencher) {
    // Load the test data
    let sample_text = std::fs::read_to_string("./assets/test-payloads/escaped_json_logs.txt").unwrap();

    let det = crate::detective::Detective::new();

    b.iter(|| {
        let _ = det.get_embedded_json(&sample_text);
    });
}
