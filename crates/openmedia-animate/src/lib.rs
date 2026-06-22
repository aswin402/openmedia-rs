use serde::{Deserialize, Serialize};
use openmedia_core::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmilAnimation {
    /// <animate> — animate a single attribute
    Animate {
        attribute_name: String,
        from: String,
        to: String,
        dur: f64,
        begin: f64,
        fill: AnimationFill,
        repeat_count: RepeatCount,
        easing: Easing,
    },
    /// <animateTransform> — animate transform attribute
    AnimateTransform {
        transform_type: TransformType,
        from: String,
        to: String,
        dur: f64,
        begin: f64,
        fill: AnimationFill,
        repeat_count: RepeatCount,
        easing: Easing,
    },
    /// <animateMotion> — animate element along a path
    AnimateMotion {
        path: String,
        dur: f64,
        begin: f64,
        fill: AnimationFill,
        repeat_count: RepeatCount,
        rotate: MotionRotate,
    },
    /// <set> — set an attribute at a point in time
    Set {
        attribute_name: String,
        to: String,
        begin: f64,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimationFill {
    Remove,
    Freeze,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RepeatCount {
    Definite(u32),
    Indefinite,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransformType {
    Translate,
    Rotate,
    Scale,
    SkewX,
    SkewY,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotionRotate {
    Auto,
    AutoReverse,
    Fixed(f64),
}

/// CSS @keyframes animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssKeyframes {
    /// Animation name
    pub name: String,
    /// Keyframe steps (percentage → properties)
    pub steps: Vec<CssKeyframeStep>,
    /// Animation duration
    pub duration: f64,
    /// Timing function
    pub timing_function: String,
    /// Animation delay
    pub delay: f64,
    /// Iteration count
    pub iteration_count: CssIterationCount,
    /// Animation direction
    pub direction: CssDirection,
    /// Fill mode
    pub fill_mode: CssFillMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssKeyframeStep {
    /// Percentage (0.0–100.0)
    pub percentage: f64,
    /// CSS properties at this step
    pub properties: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CssIterationCount {
    Count(u32),
    Infinite,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CssDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CssFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl CssKeyframes {
    /// Generate the CSS string for this animation
    pub fn to_css(&self) -> String {
        String::new()
    }
}

/// Easing function for animation timing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Easing {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseOutBounce,
    EaseInBack,
    EaseOutBack,
    EaseInElastic,
    EaseOutElastic,
    Spring { stiffness: f64, damping: f64, mass: f64 },
    CubicBezier(f64, f64, f64, f64),
}

impl Easing {
    /// Evaluate the easing function at time t (0.0–1.0) → value (0.0–1.0)
    pub fn evaluate(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseInQuad => t * t,
            Self::EaseOutQuad => t * (2.0 - t),
            Self::EaseInOutQuad => {
                if t < 0.5 { 2.0 * t * t }
                else { -1.0 + (4.0 - 2.0 * t) * t }
            }
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Self::EaseInOutCubic => {
                if t < 0.5 { 4.0 * t * t * t }
                else {
                    let t = 2.0 * t - 2.0;
                    0.5 * t * t * t + 1.0
                }
            }
            Self::EaseInExpo => {
                if t == 0.0 { 0.0 } else { (2.0_f64).powf(10.0 * (t - 1.0)) }
            }
            Self::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 - (2.0_f64).powf(-10.0 * t) }
            }
            Self::EaseInOutExpo => {
                if t == 0.0 { return 0.0; }
                if t == 1.0 { return 1.0; }
                if t < 0.5 {
                    0.5 * (2.0_f64).powf(20.0 * t - 10.0)
                } else {
                    1.0 - 0.5 * (2.0_f64).powf(-20.0 * t + 10.0)
                }
            }
            Self::EaseOutBounce => bounce_out(t),
            Self::EaseInBack => {
                let s = 1.70158;
                t * t * ((s + 1.0) * t - s)
            }
            Self::EaseOutBack => {
                let s = 1.70158;
                let t = t - 1.0;
                t * t * ((s + 1.0) * t + s) + 1.0
            }
            Self::EaseInElastic => {
                if t == 0.0 || t == 1.0 { return t; }
                let p = 0.3;
                -(2.0_f64.powf(10.0 * (t - 1.0)) * ((t - 1.0 - p / 4.0) * std::f64::consts::TAU / p).sin())
            }
            Self::EaseOutElastic => {
                if t == 0.0 || t == 1.0 { return t; }
                let p = 0.3;
                2.0_f64.powf(-10.0 * t) * ((t - p / 4.0) * std::f64::consts::TAU / p).sin() + 1.0
            }
            Self::Spring { stiffness, damping, mass } => {
                spring_evaluate(t, *stiffness, *damping, *mass)
            }
            Self::CubicBezier(x1, y1, x2, y2) => {
                cubic_bezier_evaluate(t, *x1, *y1, *x2, *y2)
            }
        }
    }

    /// Convert to CSS timing-function string
    pub fn to_css(&self) -> String {
        match self {
            Self::Linear => "linear".into(),
            Self::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({x1},{y1},{x2},{y2})")
            }
            _ => "ease".into(),
        }
    }
}

fn bounce_out(t: f64) -> f64 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}

fn spring_evaluate(t: f64, stiffness: f64, damping: f64, mass: f64) -> f64 {
    let omega = (stiffness / mass).sqrt();
    let zeta = damping / (2.0 * (stiffness * mass).sqrt());
    if zeta < 1.0 {
        let omega_d = omega * (1.0 - zeta * zeta).sqrt();
        1.0 - (-zeta * omega * t).exp() * ((zeta * omega * t / omega_d).sin() * zeta * omega / omega_d + (omega_d * t).cos())
    } else {
        1.0 - (1.0 + omega * t) * (-omega * t).exp()
    }
}

fn cubic_bezier_evaluate(t: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let mut guess = t;
    for _ in 0..8 {
        let x = cubic_bezier_x(guess, x1, x2) - t;
        let dx = cubic_bezier_dx(guess, x1, x2);
        if dx.abs() < 1e-7 { break; }
        guess -= x / dx;
    }
    cubic_bezier_y(guess, y1, y2)
}

fn cubic_bezier_x(t: f64, x1: f64, x2: f64) -> f64 {
    3.0 * (1.0 - t).powi(2) * t * x1 + 3.0 * (1.0 - t) * t.powi(2) * x2 + t.powi(3)
}

fn cubic_bezier_y(t: f64, y1: f64, y2: f64) -> f64 {
    3.0 * (1.0 - t).powi(2) * t * y1 + 3.0 * (1.0 - t) * t.powi(2) * y2 + t.powi(3)
}

fn cubic_bezier_dx(t: f64, x1: f64, x2: f64) -> f64 {
    3.0 * (1.0 - t).powi(2) * x1 + 6.0 * (1.0 - t) * t * (x2 - x1) + 3.0 * t.powi(2) * (1.0 - x2)
}

/// Timeline for composing multiple animations
#[derive(Debug, Clone)]
pub struct AnimationTimeline {
    pub mode: TimelineMode,
    pub animations: Vec<TimelineEntry>,
    pub total_duration: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimelineMode {
    Parallel,
    Sequential,
    Staggered { delay: f64 },
}

#[derive(Debug, Clone)]
pub struct TimelineEntry {
    pub element_selector: String,
    pub animation: SmilAnimation,
    pub offset: f64,
}

impl AnimationTimeline {
    pub fn new(mode: TimelineMode) -> Self {
        Self {
            mode,
            animations: Vec::new(),
            total_duration: 0.0,
        }
    }

    pub fn add(&mut self, selector: &str, animation: SmilAnimation) -> &mut Self {
        self.animations.push(TimelineEntry {
            element_selector: selector.to_string(),
            animation,
            offset: 0.0,
        });
        self.recalculate_duration();
        self
    }

    fn recalculate_duration(&mut self) {
        // Simple recalculation stub
    }

    pub fn to_svg(&self) -> String {
        String::new()
    }
}

/// Morph between two SVG paths
pub fn morph_paths(
    _from_d: &str,
    _to_d: &str,
    _steps: u32,
    _easing: &Easing,
) -> Result<Vec<String>> {
    Ok(vec![])
}

/// Pre-built animation presets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationPreset {
    FadeIn,
    FadeOut,
    SlideInLeft,
    SlideInRight,
    SlideInUp,
    SlideInDown,
    Bounce,
    Pulse,
    Spin,
    Shake,
    Wobble,
    Typewriter,
    DrawPath,
    Morph,
    GradientShift,
    ParallaxScroll,
    Stagger,
}

impl AnimationPreset {
    pub fn generate(
        &self,
        _duration: f64,
        _delay: f64,
        _easing: &Easing,
        _params: &serde_json::Value,
    ) -> Result<AnimationOutput> {
        Ok(AnimationOutput::Smil(vec![]))
    }
}

pub enum AnimationOutput {
    Smil(Vec<SmilAnimation>),
    Css(CssKeyframes),
    Combined {
        smil: Vec<SmilAnimation>,
        css: CssKeyframes,
    },
}
