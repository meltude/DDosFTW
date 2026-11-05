use std::thread;

use crate::style;

use super::random::{
    mask_account, rand_delay, random_city, random_confidence, random_count, random_date,
    random_device, random_fake_name, random_ip, random_mac, random_platforms, random_port,
    random_timezone, random_token,
};
use super::Step;

fn print_steps(steps: &[Step]) {
    for step in steps {
        println!("{} {}", style::dim("[>]"), step.message);
        thread::sleep(rand_delay(step));
    }
}

pub fn fake_hack_sequence(target: &str) {
    print_steps(&[
        Step::new(format!("resolving domain {target}..."), 600, 1000),
        Step::new(format!("target IP located: {}", random_ip()), 400, 800),
        Step::new(
            format!("scanning open ports on {target}..."),
            1200,
            2000,
        ),
        Step::new(format!("port {} found open", random_port()), 300, 700),
        Step::new("identifying server software...", 800, 1300),
        Step::new("server fingerprint: nginx/1.18.0", 500, 900),
        Step::new("spoofing MAC address...", 600, 1200),
        Step::new(format!("new MAC: {}", random_mac()), 300, 700),
        Step::new("bypassing firewall...", 1500, 2500),
        Step::new("injecting payload...", 1000, 2000),
        Step::new("root access obtained", 500, 900),
        Step::new(
            format!("accessing root directory of {target}..."),
            600,
            1200,
        ),
        Step::new("extracting data...", 1400, 2200),
        Step::new("disconnecting...", 300, 700),
    ]);
}

pub fn fake_account_demo(service: &str, account: &str) {
    let masked = mask_account(account);
    let decoy_hits = 1 + random_count(8);
    let created = random_date();
    let last_login_ip = random_ip();
    let city = random_city();
    let device = random_device();
    let display_name = random_fake_name();
    let followers = random_count(250_000);
    let token = random_token();

    print_steps(&[
        Step::new(
            format!("loading fictional profile for {service}..."),
            600,
            1000,
        ),
        Step::new(format!("selected account: {account}"), 400, 800),
        Step::new(format!("masked identity: {masked}"), 250, 500),
        Step::new("checking fictional breach records...", 700, 1100),
        Step::new(format!("{decoy_hits} decoy credential hits found"), 400, 700),
        Step::new("trying to figure out session token", 800, 1300),
        Step::new(format!("session token ID: {token}"), 500, 900),
        Step::new("bypassing two-factor prompt (visual only)...", 900, 1500),
        Step::new(format!("linked email: {masked}"), 400, 700),
        Step::new(format!("account created: {created}"), 300, 600),
        Step::new(
            format!("last login: {last_login_ip} from {city}"),
            400,
            800,
        ),
        Step::new(format!("device fingerprint: {device}"), 400, 800),
        Step::new(format!("display name guess: {display_name}"), 300, 600),
        Step::new(format!("follower count: {followers}"), 300, 600),
        Step::new("escalating to admin (not really)...", 800, 1400),
        Step::new("access level: administrator (fictional)", 400, 700),
        Step::new("hacking sequence finished without any problems!", 400, 800),
    ]);
}

pub fn fake_deanon_sequence(alias: &str) {
    let platforms = random_platforms(4).join(", ");
    let matches = 3 + random_count(5);
    let shared_avatars = 1 + random_count(3);
    let style_match = random_confidence();
    let masked_email = mask_account(&format!("{alias}@protonmail.example"));
    let decoy_hits = 1 + random_count(6);
    let first_seen = random_date();
    let probable_name = random_fake_name();
    let city = random_city();
    let device = random_device();
    let timezone = random_timezone();
    let confidence = random_confidence();

    print_steps(&[
        Step::new(
            format!("starting de-anonymisation of '{alias}' (fictional)..."),
            600,
            1000,
        ),
        Step::new(format!("scanning {platforms}..."), 900, 1500),
        Step::new(format!("{matches} matching profiles found"), 400, 800),
        Step::new(
            format!("correlating avatar hashes... {shared_avatars} shared fingerprints"),
            700,
            1200,
        ),
        Step::new(
            format!("comparing writing style... {style_match:.1}% match"),
            800,
            1300,
        ),
        Step::new(format!("recovered email: {masked_email}"), 500, 900),
        Step::new(
            format!("checking fictional breach records... {decoy_hits} decoy hits"),
            600,
            1000,
        ),
        Step::new(format!("username reused since {first_seen}"), 400, 700),
        Step::new(format!("probable name: {probable_name}"), 500, 900),
        Step::new(format!("probable location: {city}"), 400, 800),
        Step::new(format!("device fingerprint: {device}"), 400, 800),
        Step::new(format!("timezone: {timezone}"), 300, 600),
        Step::new("assembling report...", 900, 1400),
        Step::new(format!("confidence score: {confidence:.1}%"), 400, 700),
        Step::new(
            "de-anonymisation report generated (all data is fabricated)",
            500,
            900,
        ),
    ]);
}
