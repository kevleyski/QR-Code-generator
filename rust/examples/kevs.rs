/* 
 * QR Code generator demo (Rust)
 * 
 * Run this command-line program with no arguments. The program computes a bunch of demonstration
 * QR Codes and prints them to the console. Also, the SVG code for one QR Code is printed as a sample.
 * 
 * Copyright (c) Project Nayuki. (MIT License)
 * https://www.nayuki.io/page/qr-code-generator-library
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 * - The above copyright notice and this permission notice shall be included in
 *   all copies or substantial portions of the Software.
 * - The Software is provided "as is", without warranty of any kind, express or
 *   implied, including but not limited to the warranties of merchantability,
 *   fitness for a particular purpose and noninfringement. In no event shall the
 *   authors or copyright holders be liable for any claim, damages or other
 *   liability, whether in an action of contract, tort or otherwise, arising from,
 *   out of or in connection with the Software or the use or other dealings in the
 *   Software.
 */

extern crate qrcodegen;
use qrcodegen::Mask;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use qrcodegen::QrCode_MAX_VERSION;
use qrcodegen::QrCode_MIN_VERSION;

// Prints the given QrCode object to the console.
fn print_qr(qr: &QrCode) {
	let border: i32 = 4;
	for y in -border .. qr.size() + border {
		for x in -border .. qr.size() + border {
			let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
			print!("{0}{0}", c);
		}
		println!();
	}
	println!();
}


// VOD2Live https://tinyurl.com/y44qjrho
// https://vod2live.switch.tv/poc
// https://vod2live.switch.tv/slates
// https://vod2live.switch.tv/adease


// WebAssembly: https://goo.gl/2ahsEY


// Low Latency: https://tinyurl.com/yyr2rz8m
// demo: https://alhls-switchmedia.global.ssl.fastly.net/lhls/master.m3u8

// AV1: https://goo.gl/pGnNgJ

fn main() {
	let qr = QrCode::encode_text("https://goo.gl/pGnNgJ", 	 QrCodeEcc::Medium).unwrap();
	let svg = qr.to_svg_string(4);
	print_qr(&qr);
}

