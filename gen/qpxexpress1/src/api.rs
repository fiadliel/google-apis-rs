use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all QPXExpress related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_qpxexpress1 as qpxexpress1;
/// use qpxexpress1::api::TripsSearchRequest;
/// use qpxexpress1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use qpxexpress1::{QPXExpress, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = QPXExpress::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TripsSearchRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.trips().search(req)
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct QPXExpress<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for QPXExpress<S> {}

impl<'a, S> QPXExpress<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> QPXExpress<S> {
        QPXExpress {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2".to_string(),
            _base_url: "https://www.googleapis.com/qpxExpress/v1/trips/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn trips(&'a self) -> TripMethods<'a, S> {
        TripMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/qpxExpress/v1/trips/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The make, model, and type of an aircraft.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AircraftData {
    /// The aircraft code. For example, for a Boeing 777 the code would be 777.
    
    pub code: Option<String>,
    /// Identifies this as an aircraftData object. Value: the fixed string qpxexpress#aircraftData
    
    pub kind: Option<String>,
    /// The name of an aircraft, for example Boeing 777.
    
    pub name: Option<String>,
}

impl client::Part for AircraftData {}


/// An airport.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AirportData {
    /// The city code an airport is located in. For example, for JFK airport, this is NYC.
    
    pub city: Option<String>,
    /// An airport's code. For example, for Boston Logan airport, this is BOS.
    
    pub code: Option<String>,
    /// Identifies this as an airport object. Value: the fixed string qpxexpress#airportData.
    
    pub kind: Option<String>,
    /// The name of an airport. For example, for airport BOS the name is "Boston Logan International".
    
    pub name: Option<String>,
}

impl client::Part for AirportData {}


/// Information about an item of baggage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BagDescriptor {
    /// Provides the commercial name for an optional service.
    #[serde(rename="commercialName")]
    
    pub commercial_name: Option<String>,
    /// How many of this type of bag will be checked on this flight.
    
    pub count: Option<i32>,
    /// A description of the baggage.
    
    pub description: Option<Vec<String>>,
    /// Identifies this as a baggage object. Value: the fixed string qpxexpress#bagDescriptor.
    
    pub kind: Option<String>,
    /// The standard IATA subcode used to identify this optional service.
    
    pub subcode: Option<String>,
}

impl client::Part for BagDescriptor {}


/// Information about a carrier (ie. an airline, bus line, railroad, etc) that might be useful to display to an end-user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CarrierData {
    /// The IATA designator of a carrier (airline, etc). For example, for American Airlines, the code is AA.
    
    pub code: Option<String>,
    /// Identifies this as a kind of carrier (ie. an airline, bus line, railroad, etc). Value: the fixed string qpxexpress#carrierData.
    
    pub kind: Option<String>,
    /// The long, full name of a carrier. For example: American Airlines.
    
    pub name: Option<String>,
}

impl client::Part for CarrierData {}


/// Information about a city that might be useful to an end-user; typically the city of an airport.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CityData {
    /// The IATA character ID of a city. For example, for Boston this is BOS.
    
    pub code: Option<String>,
    /// The two-character country code of the country the city is located in. For example, US for the United States of America.
    
    pub country: Option<String>,
    /// Identifies this as a city, typically with one or more airports. Value: the fixed string qpxexpress#cityData.
    
    pub kind: Option<String>,
    /// The full name of a city. An example would be: New York.
    
    pub name: Option<String>,
}

impl client::Part for CityData {}


/// Detailed information about components found in the solutions of this response, including a trip's airport, city, taxes, airline, and aircraft.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    /// The aircraft that is flying between an origin and destination.
    
    pub aircraft: Option<Vec<AircraftData>>,
    /// The airport of an origin or destination.
    
    pub airport: Option<Vec<AirportData>>,
    /// The airline carrier of the aircraft flying between an origin and destination. Allowed values are IATA carrier codes.
    
    pub carrier: Option<Vec<CarrierData>>,
    /// The city that is either the origin or destination of part of a trip.
    
    pub city: Option<Vec<CityData>>,
    /// Identifies this as QPX Express response resource, including a trip's airport, city, taxes, airline, and aircraft. Value: the fixed string qpxexpress#data.
    
    pub kind: Option<String>,
    /// The taxes due for flying between an origin and a destination.
    
    pub tax: Option<Vec<TaxData>>,
}

impl client::Part for Data {}


/// Complete information about a fare used in the solution to a low-fare search query. In the airline industry a fare is a price an airline charges for one-way travel between two points. A fare typically contains a carrier code, two city codes, a price, and a fare basis. (A fare basis is a one-to-eight character alphanumeric code used to identify a fare.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FareInfo {
    /// no description provided
    #[serde(rename="basisCode")]
    
    pub basis_code: Option<String>,
    /// The carrier of the aircraft or other vehicle commuting between two points.
    
    pub carrier: Option<String>,
    /// The city code of the city the trip ends at.
    
    pub destination: Option<String>,
    /// A unique identifier of the fare.
    
    pub id: Option<String>,
    /// Identifies this as a fare object. Value: the fixed string qpxexpress#fareInfo.
    
    pub kind: Option<String>,
    /// The city code of the city the trip begins at.
    
    pub origin: Option<String>,
    /// Whether this is a private fare, for example one offered only to select customers rather than the general public.
    
    pub private: Option<bool>,
}

impl client::Part for FareInfo {}


/// A flight is a sequence of legs with the same airline carrier and flight number. (A leg is the smallest unit of travel, in the case of a flight a takeoff immediately followed by a landing at two set points on a particular carrier with a particular flight number.) The naive view is that a flight is scheduled travel of an aircraft between two points, with possibly intermediate stops, but carriers will frequently list flights that require a change of aircraft between legs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightInfo {
    /// no description provided
    
    pub carrier: Option<String>,
    /// The flight number.
    
    pub number: Option<String>,
}

impl client::Part for FlightInfo {}


/// Information about free baggage allowed on one segment of a trip.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeBaggageAllowance {
    /// A representation of a type of bag, such as an ATPCo subcode, Commercial Name, or other description.
    #[serde(rename="bagDescriptor")]
    
    pub bag_descriptor: Option<Vec<BagDescriptor>>,
    /// The maximum number of kilos all the free baggage together may weigh.
    
    pub kilos: Option<i32>,
    /// The maximum number of kilos any one piece of baggage may weigh.
    #[serde(rename="kilosPerPiece")]
    
    pub kilos_per_piece: Option<i32>,
    /// Identifies this as free baggage object, allowed on one segment of a trip. Value: the fixed string qpxexpress#freeBaggageAllowance.
    
    pub kind: Option<String>,
    /// The number of free pieces of baggage allowed.
    
    pub pieces: Option<i32>,
    /// The number of pounds of free baggage allowed.
    
    pub pounds: Option<i32>,
}

impl client::Part for FreeBaggageAllowance {}


/// Information about a leg. (A leg is the smallest unit of travel, in the case of a flight a takeoff immediately followed by a landing at two set points on a particular carrier with a particular flight number.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LegInfo {
    /// The aircraft (or bus, ferry, railcar, etc) travelling between the two points of this leg.
    
    pub aircraft: Option<String>,
    /// The scheduled time of arrival at the destination of the leg, local to the point of arrival.
    #[serde(rename="arrivalTime")]
    
    pub arrival_time: Option<String>,
    /// Whether you have to change planes following this leg. Only applies to the next leg.
    #[serde(rename="changePlane")]
    
    pub change_plane: Option<bool>,
    /// Duration of a connection following this leg, in minutes.
    #[serde(rename="connectionDuration")]
    
    pub connection_duration: Option<i32>,
    /// The scheduled departure time of the leg, local to the point of departure.
    #[serde(rename="departureTime")]
    
    pub departure_time: Option<String>,
    /// The leg destination as a city and airport.
    
    pub destination: Option<String>,
    /// The terminal the flight is scheduled to arrive at.
    #[serde(rename="destinationTerminal")]
    
    pub destination_terminal: Option<String>,
    /// The scheduled travelling time from the origin to the destination.
    
    pub duration: Option<i32>,
    /// An identifier that uniquely identifies this leg in the solution.
    
    pub id: Option<String>,
    /// Identifies this as a leg object. A leg is the smallest unit of travel, in the case of a flight a takeoff immediately followed by a landing at two set points on a particular carrier with a particular flight number. Value: the fixed string qpxexpress#legInfo.
    
    pub kind: Option<String>,
    /// A simple, general description of the meal(s) served on the flight, for example: "Hot meal".
    
    pub meal: Option<String>,
    /// The number of miles in this leg.
    
    pub mileage: Option<i32>,
    /// In percent, the published on time performance on this leg.
    #[serde(rename="onTimePerformance")]
    
    pub on_time_performance: Option<i32>,
    /// Department of Transportation disclosure information on the actual operator of a flight in a code share. (A code share refers to a marketing agreement between two carriers, where one carrier will list in its schedules (and take bookings for) flights that are actually operated by another carrier.)
    #[serde(rename="operatingDisclosure")]
    
    pub operating_disclosure: Option<String>,
    /// The leg origin as a city and airport.
    
    pub origin: Option<String>,
    /// The terminal the flight is scheduled to depart from.
    #[serde(rename="originTerminal")]
    
    pub origin_terminal: Option<String>,
    /// Whether passenger information must be furnished to the United States Transportation Security Administration (TSA) prior to departure.
    
    pub secure: Option<bool>,
}

impl client::Part for LegInfo {}


/// The number and type of passengers. Unfortunately the definition of an infant, child, adult, and senior citizen varies across carriers and reservation systems.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PassengerCounts {
    /// The number of passengers that are adults.
    #[serde(rename="adultCount")]
    
    pub adult_count: Option<i32>,
    /// The number of passengers that are children.
    #[serde(rename="childCount")]
    
    pub child_count: Option<i32>,
    /// The number of passengers that are infants travelling in the lap of an adult.
    #[serde(rename="infantInLapCount")]
    
    pub infant_in_lap_count: Option<i32>,
    /// The number of passengers that are infants each assigned a seat.
    #[serde(rename="infantInSeatCount")]
    
    pub infant_in_seat_count: Option<i32>,
    /// Identifies this as a passenger count object, representing the number of passengers. Value: the fixed string qpxexpress#passengerCounts.
    
    pub kind: Option<String>,
    /// The number of passengers that are senior citizens.
    #[serde(rename="seniorCount")]
    
    pub senior_count: Option<i32>,
}

impl client::Part for PassengerCounts {}


/// The price of one or more travel segments. The currency used to purchase tickets is usually determined by the sale/ticketing city or the sale/ticketing country, unless none are specified, in which case it defaults to that of the journey origin country.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PricingInfo {
    /// The total fare in the base fare currency (the currency of the country of origin). This element is only present when the sales currency and the currency of the country of commencement are different.
    #[serde(rename="baseFareTotal")]
    
    pub base_fare_total: Option<String>,
    /// The fare used to price one or more segments.
    
    pub fare: Option<Vec<FareInfo>>,
    /// The horizontal fare calculation. This is a field on a ticket that displays all of the relevant items that go into the calculation of the fare.
    #[serde(rename="fareCalculation")]
    
    pub fare_calculation: Option<String>,
    /// Identifies this as a pricing object, representing the price of one or more travel segments. Value: the fixed string qpxexpress#pricingInfo.
    
    pub kind: Option<String>,
    /// The latest ticketing time for this pricing assuming the reservation occurs at ticketing time and there is no change in fares/rules. The time is local to the point of sale (POS).
    #[serde(rename="latestTicketingTime")]
    
    pub latest_ticketing_time: Option<String>,
    /// The number of passengers to which this price applies.
    
    pub passengers: Option<PassengerCounts>,
    /// The passenger type code for this pricing. An alphanumeric code used by a carrier to restrict fares to certain categories of passenger. For instance, a fare might be valid only for senior citizens.
    
    pub ptc: Option<String>,
    /// Whether the fares on this pricing are refundable.
    
    pub refundable: Option<bool>,
    /// The total fare in the sale or equivalent currency.
    #[serde(rename="saleFareTotal")]
    
    pub sale_fare_total: Option<String>,
    /// The taxes in the sale or equivalent currency.
    #[serde(rename="saleTaxTotal")]
    
    pub sale_tax_total: Option<String>,
    /// Total per-passenger price (fare and tax) in the sale or equivalent currency.
    #[serde(rename="saleTotal")]
    
    pub sale_total: Option<String>,
    /// The per-segment price and baggage information.
    #[serde(rename="segmentPricing")]
    
    pub segment_pricing: Option<Vec<SegmentPricing>>,
    /// The taxes used to calculate the tax total per ticket.
    
    pub tax: Option<Vec<TaxInfo>>,
}

impl client::Part for PricingInfo {}


/// Details of a segment of a flight; a segment is one or more consecutive legs on the same flight. For example a hypothetical flight ZZ001, from DFW to OGG, would have one segment with two legs: DFW to HNL (leg 1), HNL to OGG (leg 2), and DFW to OGG (legs 1 and 2).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentInfo {
    /// The booking code or class for this segment.
    #[serde(rename="bookingCode")]
    
    pub booking_code: Option<String>,
    /// The number of seats available in this booking code on this segment.
    #[serde(rename="bookingCodeCount")]
    
    pub booking_code_count: Option<i32>,
    /// The cabin booked for this segment.
    
    pub cabin: Option<String>,
    /// In minutes, the duration of the connection following this segment.
    #[serde(rename="connectionDuration")]
    
    pub connection_duration: Option<i32>,
    /// The duration of the flight segment in minutes.
    
    pub duration: Option<i32>,
    /// The flight this is a segment of.
    
    pub flight: Option<FlightInfo>,
    /// An id uniquely identifying the segment in the solution.
    
    pub id: Option<String>,
    /// Identifies this as a segment object. A segment is one or more consecutive legs on the same flight. For example a hypothetical flight ZZ001, from DFW to OGG, could have one segment with two legs: DFW to HNL (leg 1), HNL to OGG (leg 2). Value: the fixed string qpxexpress#segmentInfo.
    
    pub kind: Option<String>,
    /// The legs composing this segment.
    
    pub leg: Option<Vec<LegInfo>>,
    /// The solution-based index of a segment in a married segment group. Married segments can only be booked together. For example, an airline might report a certain booking code as sold out from Boston to Pittsburgh, but as available as part of two married segments Boston to Chicago connecting through Pittsburgh. For example content of this field, consider the round-trip flight ZZ1 PHX-PHL ZZ2 PHL-CLT ZZ3 CLT-PHX. This has three segments, with the two outbound ones (ZZ1 ZZ2) married. In this case, the two outbound segments belong to married segment group 0, and the return segment belongs to married segment group 1.
    #[serde(rename="marriedSegmentGroup")]
    
    pub married_segment_group: Option<String>,
    /// Whether the operation of this segment remains subject to government approval.
    #[serde(rename="subjectToGovernmentApproval")]
    
    pub subject_to_government_approval: Option<bool>,
}

impl client::Part for SegmentInfo {}


/// The price of this segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentPricing {
    /// A segment identifier unique within a single solution. It is used to refer to different parts of the same solution.
    #[serde(rename="fareId")]
    
    pub fare_id: Option<String>,
    /// Details of the free baggage allowance on this segment.
    #[serde(rename="freeBaggageOption")]
    
    pub free_baggage_option: Option<Vec<FreeBaggageAllowance>>,
    /// Identifies this as a segment pricing object, representing the price of this segment. Value: the fixed string qpxexpress#segmentPricing.
    
    pub kind: Option<String>,
    /// Unique identifier in the response of this segment.
    #[serde(rename="segmentId")]
    
    pub segment_id: Option<String>,
}

impl client::Part for SegmentPricing {}


/// Information about a slice. A slice represents a traveller's intent, the portion of a low-fare search corresponding to a traveler's request to get between two points. One-way journeys are generally expressed using 1 slice, round-trips using 2. For example, if a traveler specifies the following trip in a user interface:
/// | Origin | Destination | Departure Date | | BOS | LAX | March 10, 2007 | | LAX | SYD | March 17, 2007 | | SYD | BOS | March 22, 2007 |
/// then this is a three slice trip.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SliceInfo {
    /// The duration of the slice in minutes.
    
    pub duration: Option<i32>,
    /// Identifies this as a slice object. A slice represents a traveller's intent, the portion of a low-fare search corresponding to a traveler's request to get between two points. One-way journeys are generally expressed using 1 slice, round-trips using 2. Value: the fixed string qpxexpress#sliceInfo.
    
    pub kind: Option<String>,
    /// The segment(s) constituting the slice.
    
    pub segment: Option<Vec<SegmentInfo>>,
}

impl client::Part for SliceInfo {}


/// Criteria a desired slice must satisfy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SliceInput {
    /// Slices with only the carriers in this alliance should be returned; do not use this field with permittedCarrier. Allowed values are ONEWORLD, SKYTEAM, and STAR.
    
    pub alliance: Option<String>,
    /// Departure date in YYYY-MM-DD format.
    
    pub date: Option<String>,
    /// Airport or city IATA designator of the destination.
    
    pub destination: Option<String>,
    /// Identifies this as a slice input object, representing the criteria a desired slice must satisfy. Value: the fixed string qpxexpress#sliceInput.
    
    pub kind: Option<String>,
    /// The longest connection between two legs, in minutes, you are willing to accept.
    #[serde(rename="maxConnectionDuration")]
    
    pub max_connection_duration: Option<i32>,
    /// The maximum number of stops you are willing to accept in this slice.
    #[serde(rename="maxStops")]
    
    pub max_stops: Option<i32>,
    /// Airport or city IATA designator of the origin.
    
    pub origin: Option<String>,
    /// A list of 2-letter IATA airline designators. Slices with only these carriers should be returned.
    #[serde(rename="permittedCarrier")]
    
    pub permitted_carrier: Option<Vec<String>>,
    /// Slices must depart in this time of day range, local to the point of departure.
    #[serde(rename="permittedDepartureTime")]
    
    pub permitted_departure_time: Option<TimeOfDayRange>,
    /// Prefer solutions that book in this cabin for this slice. Allowed values are COACH, PREMIUM_COACH, BUSINESS, and FIRST.
    #[serde(rename="preferredCabin")]
    
    pub preferred_cabin: Option<String>,
    /// A list of 2-letter IATA airline designators. Exclude slices that use these carriers.
    #[serde(rename="prohibitedCarrier")]
    
    pub prohibited_carrier: Option<Vec<String>>,
}

impl client::Part for SliceInput {}


/// Tax data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaxData {
    /// An identifier uniquely identifying a tax in a response.
    
    pub id: Option<String>,
    /// Identifies this as a tax data object, representing some tax. Value: the fixed string qpxexpress#taxData.
    
    pub kind: Option<String>,
    /// The name of a tax.
    
    pub name: Option<String>,
}

impl client::Part for TaxData {}


/// Tax information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaxInfo {
    /// Whether this is a government charge or a carrier surcharge.
    #[serde(rename="chargeType")]
    
    pub charge_type: Option<String>,
    /// The code to enter in the ticket's tax box.
    
    pub code: Option<String>,
    /// For government charges, the country levying the charge.
    
    pub country: Option<String>,
    /// Identifier uniquely identifying this tax in a response. Not present for unnamed carrier surcharges.
    
    pub id: Option<String>,
    /// Identifies this as a tax information object. Value: the fixed string qpxexpress#taxInfo.
    
    pub kind: Option<String>,
    /// The price of the tax in the sales or equivalent currency.
    #[serde(rename="salePrice")]
    
    pub sale_price: Option<String>,
}

impl client::Part for TaxInfo {}


/// Two times in a single day defining a time range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDayRange {
    /// The earliest time of day in HH:MM format.
    #[serde(rename="earliestTime")]
    
    pub earliest_time: Option<String>,
    /// Identifies this as a time of day range object, representing two times in a single day defining a time range. Value: the fixed string qpxexpress#timeOfDayRange.
    
    pub kind: Option<String>,
    /// The latest time of day in HH:MM format.
    #[serde(rename="latestTime")]
    
    pub latest_time: Option<String>,
}

impl client::Part for TimeOfDayRange {}


/// Trip information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TripOption {
    /// Identifier uniquely identifying this trip in a response.
    
    pub id: Option<String>,
    /// Identifies this as a trip information object. Value: the fixed string qpxexpress#tripOption.
    
    pub kind: Option<String>,
    /// Per passenger pricing information.
    
    pub pricing: Option<Vec<PricingInfo>>,
    /// The total price for all passengers on the trip, in the form of a currency followed by an amount, e.g. USD253.35.
    #[serde(rename="saleTotal")]
    
    pub sale_total: Option<String>,
    /// The slices that make up this trip's itinerary.
    
    pub slice: Option<Vec<SliceInfo>>,
}

impl client::Part for TripOption {}


/// A QPX Express search request, which will yield one or more solutions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TripOptionsRequest {
    /// Do not return solutions that cost more than this price. The alphabetical part of the price is in ISO 4217. The format, in regex, is [A-Z]{3}\d+(\.\d+)? Example: $102.07
    #[serde(rename="maxPrice")]
    
    pub max_price: Option<String>,
    /// Counts for each passenger type in the request.
    
    pub passengers: Option<PassengerCounts>,
    /// Return only solutions with refundable fares.
    
    pub refundable: Option<bool>,
    /// IATA country code representing the point of sale. This determines the "equivalent amount paid" currency for the ticket.
    #[serde(rename="saleCountry")]
    
    pub sale_country: Option<String>,
    /// The slices that make up the itinerary of this trip. A slice represents a traveler's intent, the portion of a low-fare search corresponding to a traveler's request to get between two points. One-way journeys are generally expressed using one slice, round-trips using two. An example of a one slice trip with three segments might be BOS-SYD, SYD-LAX, LAX-BOS if the traveler only stopped in SYD and LAX just long enough to change planes.
    
    pub slice: Option<Vec<SliceInput>>,
    /// The number of solutions to return, maximum 500.
    
    pub solutions: Option<i32>,
    /// IATA country code representing the point of ticketing.
    #[serde(rename="ticketingCountry")]
    
    pub ticketing_country: Option<String>,
}

impl client::Part for TripOptionsRequest {}


/// A QPX Express search response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TripOptionsResponse {
    /// Informational data global to list of solutions.
    
    pub data: Option<Data>,
    /// Identifies this as a QPX Express trip response object, which consists of zero or more solutions. Value: the fixed string qpxexpress#tripOptions.
    
    pub kind: Option<String>,
    /// An identifier uniquely identifying this response.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// A list of priced itinerary solutions to the QPX Express query.
    #[serde(rename="tripOption")]
    
    pub trip_option: Option<Vec<TripOption>>,
}

impl client::Part for TripOptionsResponse {}


/// A QPX Express search request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search trips](TripSearchCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TripsSearchRequest {
    /// A QPX Express search request. Required values are at least one adult or senior passenger, an origin, a destination, and a date.
    
    pub request: Option<TripOptionsRequest>,
}

impl client::RequestValue for TripsSearchRequest {}


/// A QPX Express search response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search trips](TripSearchCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TripsSearchResponse {
    /// Identifies this as a QPX Express API search response resource. Value: the fixed string qpxExpress#tripsSearch.
    
    pub kind: Option<String>,
    /// All possible solutions to the QPX Express search request.
    
    pub trips: Option<TripOptionsResponse>,
}

impl client::ResponseResult for TripsSearchResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *trip* resources.
/// It is not used directly, but through the [`QPXExpress`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_qpxexpress1 as qpxexpress1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use qpxexpress1::{QPXExpress, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = QPXExpress::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.trips();
/// # }
/// ```
pub struct TripMethods<'a, S>
    where S: 'a {

    hub: &'a QPXExpress<S>,
}

impl<'a, S> client::MethodsBuilder for TripMethods<'a, S> {}

impl<'a, S> TripMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of flights.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: TripsSearchRequest) -> TripSearchCall<'a, S> {
        TripSearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns a list of flights.
///
/// A builder for the *search* method supported by a *trip* resource.
/// It is not used directly, but through a [`TripMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_qpxexpress1 as qpxexpress1;
/// use qpxexpress1::api::TripsSearchRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use qpxexpress1::{QPXExpress, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = QPXExpress::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TripsSearchRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.trips().search(req)
///              .doit().await;
/// # }
/// ```
pub struct TripSearchCall<'a, S>
    where S: 'a {

    hub: &'a QPXExpress<S>,
    _request: TripsSearchRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for TripSearchCall<'a, S> {}

impl<'a, S> TripSearchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TripsSearchResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "qpxExpress.trips.search",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "search";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: TripsSearchRequest) -> TripSearchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TripSearchCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> TripSearchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


