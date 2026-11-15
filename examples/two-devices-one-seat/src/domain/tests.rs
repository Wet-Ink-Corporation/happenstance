//! The payload's shape, written out byte for byte, so a change to it is a
//! change to this file and not a silent change to every log.

use happenstance::{DomainEvent, Json};

use super::{BoxOffice, Place, RefundReason, Rule, SaleId, ShowId};

fn encoded(event: &BoxOffice) -> String {
    let bytes = event.encode(&Json).expect("the payload encodes");
    String::from_utf8(bytes.to_vec()).expect("JSON is UTF-8")
}

#[test]
fn the_payload_names_its_type_in_snake_case() {
    let show = ShowId::new("premiere").expect("a valid show");
    let sold = BoxOffice::SeatSold {
        show: show.clone(),
        sale: SaleId::mint(Place::Tablet, 1),
        buyer: "Grace".to_owned(),
        sold_by: Place::Tablet,
    };
    let refunded = BoxOffice::SaleRefunded {
        show,
        sale: SaleId::mint(Place::Phone, 1),
        reason: RefundReason {
            capacity: 3,
            sales_seen: 4,
            rule: Rule::FirstToReachCentralKeepsTheSeat,
        },
    };

    assert_eq!(
        encoded(&sold),
        r#"{"type":"seat_sold","show":"premiere","sale":"tablet-1","buyer":"Grace","sold_by":"tablet"}"#
    );
    assert_eq!(
        encoded(&refunded),
        r#"{"type":"sale_refunded","show":"premiere","sale":"phone-1","reason":{"capacity":3,"sales_seen":4,"rule":"FirstToReachCentralKeepsTheSeat"}}"#
    );
    for event in [sold, refunded] {
        let back = BoxOffice::decode(
            &Json,
            &event.event_type(),
            &event.encode(&Json).expect("encodes"),
        )
        .expect("the pinned shape decodes");
        assert_eq!(back, event);
    }
}
