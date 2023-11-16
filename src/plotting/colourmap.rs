use itertools::Itertools;
use plotly::common::ColorScalePalette;

struct Rgb {
    r: f64,
    g: f64,
    b: f64,
}

impl Rgb {
    // Internal representation of RGB strings in this crate
    // is rgb(int, int, int). Values are hard-coded so we can
    // make assumptions around layout
    fn from_rgb_string(rgb_string: String) -> Rgb {
        // cut rgb( and ) from the string
        let tuple = &rgb_string[4..rgb_string.len() - 1];
        let rgb_strings: Vec<&str> = tuple.split(',').collect();
        let rgb_vec = vec![
            rgb_strings[0].parse::<u8>().unwrap(),
            rgb_strings[1].parse::<u8>().unwrap(),
            rgb_strings[2].parse::<u8>().unwrap(),
        ];
        Rgb::from_rgb_vec(rgb_vec)
    }
    fn from_rgb_vec(rgb_vec: Vec<u8>) -> Rgb {
        // conversion factor from 0 < x < 255 to 0 < x < 1
        let div = 255.;
        let r = (rgb_vec[0] as f64) / div;
        let g = (rgb_vec[1] as f64) / div;
        let b = (rgb_vec[2] as f64) / div;
        Rgb { r, g, b }
    }
    fn as_array(&self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }
    // taken from Agoston's Computer Graphics and Geometric Modeling:
    // Implementation and Algorithms p. 303, with H -> [0, 360] and S,V -> [0, 1]
    fn to_hsv(&self) -> Hsv {
        let max = self
            .as_array()
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min = self.as_array().iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let v = max;
        let s = if max == 0. { 0. } else { (max - min) / max };
        let mut h = if s == 0. {
            f64::NAN
        } else {
            let delta = max - min;
            // colour between yellow and magenta
            if self.r == max {
                (self.g - self.b) / delta
            }
            // colour between cyan and yellow
            else if self.g == max {
                2. + (self.b - self.r) / delta
            }
            // colour between magenta and cyan
            else {
                4. + (self.r - self.g) / delta
            }
        };
        // convert to degrees
        h *= 60.;
        // ensure that 0 < h < 360
        if h < 0. {
            h += 360.
        }
        Hsv { h, s, v }
    }
}

struct Hsv {
    h: f64,
    s: f64,
    v: f64,
}

// return the colour that the normalised value is located at along
// the provided colourmap
#[allow(unused_variables)]
pub fn interpolate_cmap(cmap: ColorScalePalette, cmap_loc: f64) //-> String
{
    let cmap_strings = ColorScalePaletteRawValues::get_values(cmap);
    let mut cmap_rgb_values: Vec<Rgb> = Vec::with_capacity(cmap_strings.len());
    for colour in cmap_strings {
        cmap_rgb_values.push(Rgb::from_rgb_string(colour));
    }
}

fn convert_to_string(vector: Vec<&str>) -> Vec<String> {
    vector.iter().map(|s| s.to_string()).collect_vec()
}

enum ColorScalePaletteRawValues {
    Greys,
    YlGnBu,
    Greens,
    YlOrRd,
    Bluered,
    RdBu,
    Reds,
    Blues,
    Picnic,
    Rainbow,
    Portland,
    Jet,
    Hot,
    Blackbody,
    Earth,
    Electric,
    Viridis,
    Cividis,
}

impl ColorScalePaletteRawValues {
    fn get_values(cmap: ColorScalePalette) -> Vec<String> {
        match cmap {
            ColorScalePalette::Greys => convert_to_string(vec![
                "rgb(255,255,255)",
                "rgb(240,240,240)",
                "rgb(217,217,217)",
                "rgb(189,189,189)",
                "rgb(150,150,150)",
                "rgb(115,115,115)",
                "rgb(82,82,82)",
                "rgb(37,37,37)",
                "rgb(0,0,0)",
            ]),
            ColorScalePalette::YlGnBu => convert_to_string(vec![
                "rgb(8,29,88)",
                "rgb(37,52,148)",
                "rgb(34,94,168)",
                "rgb(29,145,192)",
                "rgb(65,182,196)",
                "rgb(127,205,187)",
                "rgb(199,233,180)",
                "rgb(237,248,217)",
                "rgb(255,255,217)",
            ]),
            ColorScalePalette::Greens => convert_to_string(vec![
                "rgb(0,68,27)",
                "rgb(0,109,44)",
                "rgb(35,139,69)",
                "rgb(65,171,93)",
                "rgb(116,196,118)",
                "rgb(161,217,155)",
                "rgb(199,233,192)",
                "rgb(229,245,224)",
                "rgb(247,252,245)",
            ]),
            ColorScalePalette::YlOrRd => convert_to_string(vec![
                "rgb(128,0,38)",
                "rgb(189,0,38)",
                "rgb(227,26,28)",
                "rgb(252,78,42)",
                "rgb(253,141,60)",
                "rgb(254,178,76)",
                "rgb(254,217,118)",
                "rgb(255,237,160)",
                "rgb(255,255,204)",
            ]),
            ColorScalePalette::Bluered => convert_to_string(vec!["rgb(0,0,255)", "rgb(255,0,0)"]),
            ColorScalePalette::RdBu => convert_to_string(vec![
                "rgb(103,0,31)",
                "rgb(178,24,43)",
                "rgb(214,96,77)",
                "rgb(244,165,130)",
                "rgb(253,219,199)",
                "rgb(247,247,247)",
                "rgb(209,229,240)",
                "rgb(146,197,222)",
                "rgb(67,147,195)",
                "rgb(33,102,172)",
                "rgb(5,48,97)",
            ]),
            ColorScalePalette::Reds => convert_to_string(vec![
                "rgb(220,220,220)",
                "rgb(245,195,157)",
                "rgb(245,160,105)",
                "rgb(178,10,28)",
            ]),
            ColorScalePalette::Blues => convert_to_string(vec![
                "rgb(5,10,172)",
                "rgb(40,60,190)",
                "rgb(70,100,245)",
                "rgb(90,120,245)",
                "rgb(106,137,247)",
                "rgb(220,220,220)",
            ]),
            ColorScalePalette::Picnic => convert_to_string(vec![
                "rgb(0,0,255)",
                "rgb(51,153,255)",
                "rgb(102,204,255)",
                "rgb(153,204,255)",
                "rgb(204,204,255)",
                "rgb(255,255,255)",
                "rgb(255,204,255)",
                "rgb(255,153,255)",
                "rgb(255,102,204)",
                "rgb(255,102,102)",
                "rgb(255,0,0)",
            ]),
            ColorScalePalette::Rainbow => convert_to_string(vec![
                "rgb(150,0,90)",
                "rgb(0,0,200)",
                "rgb(0,25,255)",
                "rgb(0,152,255)",
                "rgb(44,255,150)",
                "rgb(151,255,0)",
                "rgb(255,234,0)",
                "rgb(255,111,0)",
                "rgb(255,0,0)",
            ]),
            ColorScalePalette::Portland => convert_to_string(vec![
                "rgb(12,51,131)",
                "rgb(10,136,186)",
                "rgb(242,211,56)",
                "rgb(242,143,56)",
                "rgb(217,30,30)",
            ]),
            ColorScalePalette::Jet => convert_to_string(vec![
                "rgb(0,0,131)",
                "rgb(0,60,170)",
                "rgb(5,255,255)",
                "rgb(255,255,0)",
                "rgb(250,0,0)",
                "rgb(128,0,0)",
            ]),
            ColorScalePalette::Hot => convert_to_string(vec![
                "rgb(0,0,0)",
                "rgb(230,0,0)",
                "rgb(255,210,0)",
                "rgb(255,255,255)",
            ]),
            ColorScalePalette::Blackbody => convert_to_string(vec![
                "rgb(0,0,0)",
                "rgb(230,0,0)",
                "rgb(230,210,0)",
                "rgb(255,255,255)",
                "rgb(160,200,255)",
            ]),
            ColorScalePalette::Earth => convert_to_string(vec![
                "rgb(161, 105, 40)",
                "rgb(189, 146, 90)",
                "rgb(214, 189, 141)",
                "rgb(237, 234, 194)",
                "rgb(181, 200, 184)",
                "rgb(121, 167, 172)",
                "rgb(40, 135, 161)",
            ]),
            ColorScalePalette::Electric => convert_to_string(vec![
                "rgb(0,0,0)",
                "rgb(30,0,100)",
                "rgb(120,0,100)",
                "rgb(160,90,0)",
                "rgb(230,200,0)",
                "rgb(255,250,220)",
            ]),
            ColorScalePalette::Viridis => convert_to_string(vec![
                "rgb(68,1,84)",
                "rgb(72,24,106)",
                "rgb(71,45,123)",
                "rgb(66,64,134)",
                "rgb(59,82,139)",
                "rgb(51,99,141)",
                "rgb(44,114,142)",
                "rgb(38,130,142)",
                "rgb(33,145,140)",
                "rgb(31,160,136)",
                "rgb(40,174,128)",
                "rgb(63,188,115)",
                "rgb(94,201,98)",
                "rgb(132,212,75)",
                "rgb(173,220,48)",
                "rgb(216,226,25)",
                "rgb(253,231,37)",
            ]),
            ColorScalePalette::Cividis => convert_to_string(vec![
                "rgb(0,32,76)",
                "rgb(0,42,102)",
                "rgb(0,52,110)",
                "rgb(39,63,108)",
                "rgb(60,74,107)",
                "rgb(76,85,107)",
                "rgb(91,95,109)",
                "rgb(104,106,112)",
                "rgb(117,117,117)",
                "rgb(131,129,120)",
                "rgb(146,140,120)",
                "rgb(161,152,118)",
                "rgb(176,165,114)",
                "rgb(192,177,109)",
                "rgb(209,191,102)",
                "rgb(225,204,92)",
                "rgb(243,219,79)",
                "rgb(255,233,69)",
            ]),
        }
    }
}

// TODO tests
#[cfg(test)]
mod test {

use super::*;

// Helper functions

// Tests

}