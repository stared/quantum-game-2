pub use super::dimensions::*;
use crate::{cx, enumerable::enumerate_two, operator, Complex, Dims, Enumerable, Operator};
use core::f32::consts::PI;
use wasm_bindgen::prelude::*;

const TAU: f32 = PI * 2.0;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Angle {
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}

impl Angle {
    pub fn rot45(self) -> Self {
        match self {
            Angle::Right => Angle::UpRight,
            Angle::UpRight => Angle::Up,
            Angle::Up => Angle::UpLeft,
            Angle::UpLeft => Angle::Left,
            Angle::Left => Angle::DownLeft,
            Angle::DownLeft => Angle::Down,
            Angle::Down => Angle::DownRight,
            Angle::DownRight => Angle::Right,
        }
    }

    pub fn rot90(self) -> Self {
        match self {
            Angle::Up => Angle::Left,
            Angle::UpLeft => Angle::DownLeft,
            Angle::Left => Angle::Down,
            Angle::DownLeft => Angle::DownRight,
            Angle::Down => Angle::Right,
            Angle::DownRight => Angle::UpRight,
            Angle::Right => Angle::Up,
            Angle::UpRight => Angle::UpLeft,
        }
    }

    pub fn rot180(self) -> Self {
        match self {
            Angle::Left => Angle::Right,
            Angle::DownLeft => Angle::UpRight,
            Angle::Down => Angle::Up,
            Angle::DownRight => Angle::UpLeft,
            Angle::Right => Angle::Left,
            Angle::UpRight => Angle::DownLeft,
            Angle::Up => Angle::Down,
            Angle::UpLeft => Angle::DownRight,
        }
    }

    pub fn rot225(self) -> Self {
        match self {
            Angle::Right => Angle::DownRight,
            Angle::UpRight => Angle::Right,
            Angle::Up => Angle::UpRight,
            Angle::UpLeft => Angle::Up,
            Angle::Left => Angle::UpLeft,
            Angle::DownLeft => Angle::Left,
            Angle::Down => Angle::DownLeft,
            Angle::DownRight => Angle::Down,
        }
    }

    pub fn rot270(self) -> Self {
        match self {
            Angle::Down => Angle::Left,
            Angle::DownRight => Angle::DownLeft,
            Angle::Right => Angle::Down,
            Angle::UpRight => Angle::DownRight,
            Angle::Up => Angle::Right,
            Angle::UpLeft => Angle::UpRight,
            Angle::Left => Angle::Up,
            Angle::DownLeft => Angle::UpLeft,
        }
    }

    pub fn as_direction(self) -> Option<Direction> {
        Some(match self {
            Angle::Left => Direction::Right,
            Angle::Down => Direction::Up,
            Angle::Right => Direction::Left,
            Angle::Up => Direction::Down,
            _ => return None,
        })
    }

    /// Angle around the circle, in 0..1 range.
    pub fn tau(self) -> f32 {
        match self {
            Angle::Right => 0.0 / 8.0,
            Angle::UpRight => 1.0 / 8.0,
            Angle::Up => 2.0 / 8.0,
            Angle::UpLeft => 3.0 / 8.0,
            Angle::Left => 4.0 / 8.0,
            Angle::DownLeft => 5.0 / 8.0,
            Angle::Down => 6.0 / 8.0,
            Angle::DownRight => 7.0 / 8.0,
        }
    }

    /// Angle in radians
    pub fn rad(self) -> f32 {
        self.tau() * TAU
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Wall,
    Gate,
    Laser(Direction),
    NonLinearCrystal,
    Mirror(Angle),
    BeamSplitter(Angle, f32),
    PolarizingBeamSplitter(Angle),
    CornerCube,
    Detector(Direction),
    Rock,
    Mine,
    Absorber(f32),
    DetectorFour,
    Polarizer(Angle),
    QuarterWavePlate(Angle),
    HalfWavePlate(Angle),
    SugarSolution(f32),
    FaradayRotator(Direction, f32),
    Glass,
    VacuumJar,
}

impl Element {
    pub fn operator(&self) -> Operator<DimDirPol> {
        match *self {
            Element::Wall
            | Element::Rock
            | Element::Gate
            | Element::Detector(_) // TODO: Direction
            | Element::Mine
            | Element::Laser(_)
            | Element::DetectorFour => attenuator(0.0),
            Element::Mirror(angle) => mirror(angle),
            Element::BeamSplitter(angle, split) => beamsplitter(angle, split),
            Element::PolarizingBeamSplitter(angle) => polarizing_beamsplitter(angle),
            Element::CornerCube => corner_cube(),
            Element::Absorber(absorption) => attenuator(absorption),
            Element::Polarizer(angle) => polarizer(angle),
            Element::QuarterWavePlate(angle) => phase_plate(angle, 0.25),
            Element::HalfWavePlate(angle) => phase_plate(angle, 0.5),
            Element::SugarSolution(pol_rotation) => sugar_solution(pol_rotation),
            Element::FaradayRotator(dir, pol_rotation) => faraday_rotator(dir, pol_rotation),
            Element::Glass => amplitude_intensity(1.0, 0.25),
            Element::VacuumJar => amplitude_intensity(1.0, -0.25),
            Element::NonLinearCrystal => todo!(),
        }
    }
}

// ELEMS
fn attenuator(attenuation: f32) -> Operator<DimDirPol> {
    amplitude_intensity(attenuation.sqrt(), 0.0)
}

fn mirror(angle: Angle) -> Operator<DimDirPol> {
    reflect_from_plane_direction(angle).outer(&reflect_phase_from_denser())
}

fn beamsplitter(angle: Angle, split: f32) -> Operator<DimDirPol> {
    let reflectance = reflect_from_plane_direction(angle).outer(&reflect_phase_from_denser())
        * cx(0.0, split.sqrt());
    let transmittance = beamsplitter_transmittion_directions(angle)
        .outer(&Operator::<DimPol>::identity())
        * cx((1.0 - split).sqrt(), 0.0);
    reflectance + transmittance
}

fn polarizing_beamsplitter(angle: Angle) -> Operator<DimDirPol> {
    let proj_h = Operator::indicator(H);
    let proj_v = Operator::indicator(V);

    Operator::<DimDir>::identity().outer(&proj_h)
        + &reflect_from_plane_direction(angle.rot45()).outer(&proj_v)
}

fn polarizer(angle: Angle) -> Operator<DimDirPol> {
    diode(Direction::Right).outer(&projection_matrix(angle.rad()))
        + diode(Direction::Up).outer(&projection_matrix(angle.rot90().rad()))
        + diode(Direction::Left).outer(&projection_matrix(angle.rot180().rad()))
        + diode(Direction::Down).outer(&projection_matrix(angle.rot270().rad()))
}

fn corner_cube() -> Operator<DimDirPol> {
    operator![
        (RIGHT, LEFT) => Complex::ONE,
        (LEFT, RIGHT) => Complex::ONE,
        (UP, DOWN) => Complex::ONE,
        (DOWN, UP) => Complex::ONE,
    ]
    .outer(&Operator::<DimPol>::identity())
}

fn phase_plate(angle: Angle, phase: f32) -> Operator<DimDirPol> {
    phase_plate_direction(Direction::Right, angle.rad(), phase)
        + phase_plate_direction(Direction::Up, angle.rot90().rad(), phase)
}

fn sugar_solution(pol_rotation: f32) -> Operator<DimDirPol> {
    Operator::<DimDir>::identity().outer(&rotation_matrix(pol_rotation * TAU))
}

fn faraday_rotator(dir: Direction, pol_rotation: f32) -> Operator<DimDirPol> {
    diode(dir).outer(&rotation_matrix(pol_rotation * TAU))
        + diode(dir.opposite()).outer(&rotation_matrix(-pol_rotation * TAU))
}

// OPS

fn phase_plate_direction(dir: Direction, pol_rotation: f32, phase: f32) -> Operator<DimDirPol> {
    let shift1 = phase_shift_for_real_eigenvectors(pol_rotation * TAU, 0.0, phase);
    let shift2 = phase_shift_for_real_eigenvectors(-pol_rotation * TAU, 0.0, phase);
    diode(dir).outer(&shift1) + diode(dir.opposite()).outer(&shift2)
}

fn beamsplitter_transmittion_directions(angle: Angle) -> Operator<DimDir> {
    match angle {
        Angle::Right | Angle::Left => operator![
            (UP, UP) => Complex::ONE,
            (DOWN, DOWN) => Complex::ONE,
        ],
        Angle::Up | Angle::Down => operator![
            (LEFT, LEFT) => Complex::ONE,
            (RIGHT, RIGHT) => Complex::ONE,
        ],
        _ => Operator::identity(),
    }
}

fn reflect_from_plane_direction(angle: Angle) -> Operator<DimDir> {
    match angle {
        Angle::Right | Angle::Left => operator![
            (UP, DOWN) => Complex::ONE,
            (DOWN, UP) => Complex::ONE,
        ],
        Angle::Up | Angle::Down => operator![
            (LEFT, RIGHT) => Complex::ONE,
            (RIGHT, LEFT) => Complex::ONE,
        ],
        Angle::UpRight | Angle::DownLeft => operator![
            (RIGHT, UP) => Complex::ONE,
            (UP, RIGHT) => Complex::ONE,
            (LEFT, DOWN) => Complex::ONE,
            (DOWN, LEFT) => Complex::ONE,
        ],
        Angle::UpLeft | Angle::DownRight => operator![
            (RIGHT, DOWN) => Complex::ONE,
            (DOWN, RIGHT) => Complex::ONE,
            (LEFT, UP) => Complex::ONE,
            (UP, LEFT) => Complex::ONE,

        ],
    }
}

fn reflect_phase_from_denser() -> Operator<DimPol> {
    operator![
        (H, H) => cx(1.0, 0.0),
        (V, V) => cx(-1.0, 0.0),
    ]
}

/// A 2d matrix, phase shift between projections. For phase plate.
/// @param alpha An angle, in radians, i.e. from the range [0, Tau].
/// @param phase Phase shift for angle as for the main state, [0, 1].
/// @param phaseOrthogonal Phase shift for for the orthogonal state, [0, 1].
/// @type_param D dimension of size 2, e.g. spin or polarization.
fn phase_shift_for_real_eigenvectors<D: Dims + Enumerable>(
    alpha: f32,
    phase: f32,
    phase_ortho: f32,
) -> Operator<D>
where
    D::Iter: Iterator<Item = D>,
{
    (projection_matrix(alpha) * Complex::from_polar(1.0, phase * TAU))
        + (projection_matrix(alpha + 0.25 * TAU)) * Complex::from_polar(1.0, phase_ortho * TAU)
}

/// An omnidirectional operator multiplying by a complex number.
/// @param r Absolute value of amplitide multipier. E.g. Math.SQRT1_2 for
/// @param rot Phase multiplier, in TAU (from range: [0,1]).
fn amplitude_intensity(r: f32, rot: f32) -> Operator<DimDirPol> {
    Operator::uniform_scale(Complex::from_polar(r, TAU * rot))
}

/// An auxiliary operation for constructing other directional operators.
/// @param dir Direction
/// @returns Operator with dimensions [Direction].
fn diode(dir: Direction) -> Operator<Hlist![Direction]> {
    operator![(hlist![dir], hlist![dir]) => Complex::ONE]
}

fn projection_matrix<D: Dims + Enumerable>(alpha: f32) -> Operator<D>
where
    D::Iter: Iterator<Item = D>,
{
    let (a, b) = enumerate_two::<D>()
        .expect("projection_matrix requires a dimension with exactly two values");
    let sin = cx(alpha.sin(), 0.0);
    let cos = cx(alpha.cos(), 0.0);
    operator![
        (a.clone(), a.clone()) => cos * cos,
        (a.clone(), b.clone()) => cos * sin,
        (b.clone(), a) => cos * sin,
        (b.clone(), b) => sin * sin,
    ]
}

fn rotation_matrix<D: Dims + Enumerable>(alpha: f32) -> Operator<D>
where
    D::Iter: Iterator<Item = D>,
{
    let (a, b) =
        enumerate_two::<D>().expect("rotation_matrix requires a dimension with exactly two values");
    let sin = cx(alpha.sin(), 0.0);
    let cos = cx(alpha.cos(), 0.0);
    operator![
        (a.clone(), a.clone()) => cos,
        (a.clone(), b.clone()) => -sin,
        (b.clone(), a) => sin,
        (b.clone(), b) => cos,
    ]
}

#[cfg(test)]
mod tests {
    use super::Element;
    use crate::photons::dimensions::*;
    use crate::{cx, vector, Angle, Complex, Coord, Direction, SinglePhotonDims, Vector};
    use approx::assert_ulps_eq;
    use core::f32::consts::FRAC_1_SQRT_2;

    const H: Polarization = Polarization::H;
    const V: Polarization = Polarization::V;

    fn photon(
        x: u16,
        y: u16,
        dir: Direction,
        pol: Polarization,
        cx: Complex,
    ) -> Vector<SinglePhotonDims> {
        vector![
            hlist![Coord {x, y}, dir, pol] => cx
        ]
    }

    fn origin_photon(dir: Direction, pol: Polarization, cx: Complex) -> Vector<SinglePhotonDims> {
        photon(0, 0, dir, pol, cx)
    }

    fn directed_photon(dir: Direction) -> Vector<SinglePhotonDims> {
        photon(0, 0, dir, H, Complex::ONE)
    }

    fn default_photon() -> Vector<SinglePhotonDims> {
        photon(0, 0, Direction::Right, H, Complex::ONE)
    }

    fn default_mul(element: Element) -> Vector<SinglePhotonDims> {
        photon_mul(element, &default_photon())
    }

    fn photon_mul(element: Element, photon: &Vector<SinglePhotonDims>) -> Vector<SinglePhotonDims> {
        element.operator().mul_vec_partial(photon)
    }

    #[test]
    fn test_wall() {
        assert_eq!(default_mul(Element::Wall), vector![])
    }

    #[test]
    fn test_gate() {
        assert_eq!(default_mul(Element::Gate), vector![])
    }

    #[test]
    fn test_laser() {
        assert_eq!(default_mul(Element::Laser(Direction::Right)), vector![])
    }

    // #[test]
    // fn test_non_linear_crystal() {
    //     assert_eq!(test_partial_mul(Element::NonLinearCrystal), vector![])
    // }

    #[test]
    fn test_mirror() {
        assert_eq!(default_mul(Element::Mirror(Angle::Right)), vector![]);
        assert_eq!(
            default_mul(Element::Mirror(Angle::Up)),
            origin_photon(Direction::Left, H, Complex::ONE)
        );
        assert_eq!(
            default_mul(Element::Mirror(Angle::DownRight)),
            origin_photon(Direction::Down, H, Complex::ONE)
        );
        assert_eq!(
            default_mul(Element::Mirror(Angle::DownLeft)),
            origin_photon(Direction::Up, H, Complex::ONE)
        );
    }

    #[test]
    fn test_beam_splitter() {
        assert_eq!(
            default_mul(Element::BeamSplitter(Angle::Right, 0.5)),
            vector![]
        );
        assert_ulps_eq!(
            default_mul(Element::BeamSplitter(Angle::Up, 0.5)),
            origin_photon(Direction::Left, H, cx(0.0, FRAC_1_SQRT_2))
                + origin_photon(Direction::Right, H, cx(FRAC_1_SQRT_2, 0.0))
        );
        assert_eq!(
            default_mul(Element::BeamSplitter(Angle::DownRight, 0.5)),
            origin_photon(Direction::Down, H, cx(0.0, FRAC_1_SQRT_2))
                + origin_photon(Direction::Right, H, cx(FRAC_1_SQRT_2, 0.0))
        );
    }

    #[test]
    fn test_polarizing_beam_splitter() {
        let h_photon = origin_photon(Direction::Right, H, Complex::ONE);
        let v_photon = origin_photon(Direction::Right, V, Complex::ONE);
        let mix_photon = &h_photon + &v_photon;

        assert_eq!(
            photon_mul(Element::PolarizingBeamSplitter(Angle::Right), &h_photon),
            h_photon,
        );
        assert_eq!(
            photon_mul(Element::PolarizingBeamSplitter(Angle::Up), &h_photon),
            h_photon,
        );
        assert_eq!(
            photon_mul(Element::PolarizingBeamSplitter(Angle::Right), &v_photon),
            origin_photon(Direction::Up, V, Complex::ONE),
        );
        assert_eq!(
            photon_mul(Element::PolarizingBeamSplitter(Angle::Up), &v_photon),
            origin_photon(Direction::Down, V, Complex::ONE),
        );
        assert_eq!(
            photon_mul(Element::PolarizingBeamSplitter(Angle::Right), &mix_photon),
            &h_photon + origin_photon(Direction::Up, V, Complex::ONE),
        );
        assert_eq!(
            photon_mul(Element::PolarizingBeamSplitter(Angle::Up), &mix_photon),
            &h_photon + origin_photon(Direction::Down, V, Complex::ONE),
        );
    }

    #[test]
    fn test_corner_cube() {
        assert_eq!(
            photon_mul(Element::CornerCube, &directed_photon(Direction::Up)),
            directed_photon(Direction::Down)
        );
        assert_eq!(
            photon_mul(Element::CornerCube, &directed_photon(Direction::Down)),
            directed_photon(Direction::Up)
        );
        assert_eq!(
            photon_mul(Element::CornerCube, &directed_photon(Direction::Left)),
            directed_photon(Direction::Right)
        );
        assert_eq!(
            photon_mul(Element::CornerCube, &directed_photon(Direction::Right)),
            directed_photon(Direction::Left)
        );
    }

    #[test]
    fn test_detector() {
        assert_eq!(default_mul(Element::Detector(Direction::Right)), vector![])
    }

    #[test]
    fn test_rock() {
        assert_eq!(default_mul(Element::Rock), vector![])
    }

    #[test]
    fn test_mine() {
        assert_eq!(default_mul(Element::Mine), vector![])
    }

    #[test]
    fn test_absorber() {
        assert_eq!(
            default_mul(Element::Absorber(0.5)),
            origin_photon(Direction::Right, H, cx(FRAC_1_SQRT_2, 0.0))
        )
    }

    #[test]
    fn test_detector_four() {
        assert_eq!(default_mul(Element::DetectorFour), vector![])
    }

    #[test]
    fn test_polarizer() {
        let photon_45deg = origin_photon(Direction::Right, H, cx(0.5, 0.0))
            + origin_photon(Direction::Right, V, cx(0.5, 0.0));

        assert_eq!(default_mul(Element::Polarizer(Angle::Up)), vector![]);
        assert_eq!(default_mul(Element::Polarizer(Angle::Down)), vector![]);
        assert_eq!(
            default_mul(Element::Polarizer(Angle::Right)),
            default_photon()
        );
        assert_eq!(
            default_mul(Element::Polarizer(Angle::Left)),
            default_photon()
        );
        assert_ulps_eq!(
            default_mul(Element::Polarizer(Angle::UpRight)),
            photon_45deg
        );
    }

    #[test]
    fn test_quarter_wave_plate() {
        assert_eq!(
            default_mul(Element::QuarterWavePlate(Angle::Right)),
            default_photon()
        );
        assert_eq!(
            default_mul(Element::QuarterWavePlate(Angle::UpRight)),
            // TODO: verify this and replace constants with expected formula
            origin_photon(Direction::Right, H, cx(0.04865742, 0.9513424))
                + origin_photon(Direction::Right, V, cx(-0.21515092, 0.21515106))
        );
    }

    #[test]
    fn test_half_wave_plate() {
        assert_eq!(
            default_mul(Element::HalfWavePlate(Angle::Right)),
            default_photon()
        );
        assert_ulps_eq!(
            default_mul(Element::HalfWavePlate(Angle::UpRight)),
            // TODO: verify this and replace constants with expected formula
            origin_photon(Direction::Right, H, cx(-0.9026849, 0.0))
                + origin_photon(Direction::Right, V, cx(-0.43030196, 0.0))
        );
    }

    #[test]
    fn test_sugar_solution() {
        assert_eq!(
            default_mul(Element::SugarSolution(0.125)),
            origin_photon(Direction::Right, H, cx(FRAC_1_SQRT_2, 0.0))
                + origin_photon(Direction::Right, V, cx(-FRAC_1_SQRT_2, 0.0))
        )
    }

    #[test]
    fn test_faraday_rotator() {
        assert_eq!(
            default_mul(Element::FaradayRotator(Direction::Right, 0.125)),
            origin_photon(Direction::Right, H, cx(FRAC_1_SQRT_2, 0.0))
                + origin_photon(Direction::Right, V, cx(-FRAC_1_SQRT_2, 0.0))
        );
        assert_eq!(
            default_mul(Element::FaradayRotator(Direction::Up, 0.125)),
            vector![],
        );
    }

    #[test]
    fn test_glass() {
        assert_ulps_eq!(
            default_mul(Element::Glass),
            origin_photon(Direction::Right, H, cx(0.0, 1.0))
        )
    }

    #[test]
    fn test_vacuum_jar() {
        assert_ulps_eq!(
            default_mul(Element::VacuumJar),
            origin_photon(Direction::Right, H, cx(0.0, -1.0))
        )
    }
}
