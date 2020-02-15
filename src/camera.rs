// use crate::clamp::*;
use nalgebra::*;

#[derive(Debug, Copy, Clone)]
pub struct Deg<T>(pub T);

#[derive(Debug, Copy, Clone)]
pub struct Rad<T>(pub T);

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CameraTransform<T>
where
    T: nalgebra::Scalar,
{
    pub position: Point3<T>,
    pub yaw: Rad<T>,
    pub pitch: Rad<T>,
    pub fovy: Rad<T>,
}

impl<T> CameraTransform<T>
where
    T: nalgebra::Scalar + nalgebra::RealField

{
    fn interpolate(self, other: Self, t: T) -> Self {
        let s = T::one() - t;
        Self {
            position: self.position.to_vec() * s + other.position.to_vec() * t,
            yaw: self.yaw * s + other.yaw * t,
            pitch: self.pitch * s + other.pitch * t,
            fovy: self.fovy * s + other.fovy * t,
        }
    }

    #[inline]
    fn pitch_range() -> (Rad<T>, Rad<T>) {
        (Rad::from(Deg(-90.0)), Rad::from(Deg(90.0)))
    }

    #[inline]
    fn fovy_range() -> (Rad<T>, Rad<T>) {
        (Rad::from(Deg(10.0)), Rad::from(Deg(120.0)))
    }

    #[inline]
    pub fn update(&mut self, delta: &CameraDelta<T>) {
        *self = CameraTransform {
            // Direct delta_position along yaw angle.
            position: self.position
                + Quaternion::from_axis_angle(Vector3::unit_y(), self.yaw)
                    * delta.position
                    * delta.time,
            yaw: (self.yaw + delta.yaw * delta.time),
            pitch: (self.pitch + delta.pitch * delta.time).clamp_range(Self::pitch_range()),
            fovy: (self.fovy + delta.fovy * delta.time).clamp_range(Self::fovy_range()),
        };
    }

    #[inline]
    pub fn correction(&self) -> CameraCorrection<T> {
        CameraCorrection {
            delta_yaw: (self.yaw % Rad::full_turn()) - self.yaw,
        }
    }

    #[inline]
    pub fn correct(&mut self, correction: &CameraCorrection<T>) {
        self.yaw += correction.delta_yaw;
    }

    #[inline]
    pub fn rot_to_parent(&self) -> Quaternion<T> {
        Quaternion::from_axis_angle(Vector3::unit_y(), self.yaw)
            * Quaternion::from_axis_angle(Vector3::unit_x(), self.pitch)
    }

    #[inline]
    pub fn pos_to_parent(&self) -> Matrix4<T> {
        Matrix4::from_translation(self.position.to_vec()) * Matrix4::from(self.rot_to_parent())
    }

    #[inline]
    pub fn rot_from_parent(&self) -> Quaternion<T> {
        Quaternion::from_axis_angle(Vector3::unit_x(), -self.pitch)
            * Quaternion::from_axis_angle(Vector3::unit_y(), -self.yaw)
    }

    #[inline]
    pub fn pos_from_parent(&self) -> Matrix4<T> {
        Matrix4::from(self.rot_from_parent()) * Matrix4::from_translation(-self.position.to_vec())
    }
}

#[derive(Debug)]
pub struct CameraCorrection<T> {
    pub delta_yaw: Rad<T>,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CameraProperties<T>
where
    T: nalgebra::Scalar,
{
    pub z0: T,
    pub z1: T,
    pub positional_velocity: T,
    pub angular_velocity: T,
    pub zoom_velocity: T,
}

#[derive(Debug, Copy, Clone)]
pub struct CameraDelta<T>
where
    T: nalgebra::Scalar,
{
    pub time: T,
    pub position: Vector3<T>,
    pub yaw: Rad<T>,
    pub pitch: Rad<T>,
    pub fovy: Rad<T>,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Camera<T>
where
    T: nalgebra::Scalar,
{
    pub properties: CameraProperties<T>,
    pub transform: CameraTransform<T>,
}

impl<T> Camera<T>
where
    T: nalgebra::Scalar,
{
    #[inline]
    pub fn update(&mut self, delta: &CameraDelta<T>) {
        self.transform.update(&CameraDelta {
            time: delta.time,
            position: delta.position * self.properties.positional_velocity,
            yaw: delta.yaw * self.properties.angular_velocity,
            pitch: delta.pitch * self.properties.angular_velocity,
            fovy: delta.fovy * self.properties.zoom_velocity,
        })
    }

    #[inline]
    pub fn interpolate(a: Self, b: Self, t: T) -> Camera<T> {
        Camera {
            properties: b.properties,
            transform: CameraTransform::interpolate(a.transform, b.transform, t),
        }
    }
}

#[derive(Debug)]
pub struct SmoothCamera<T>
where
    T: nalgebra::Scalar,
{
    pub properties: CameraProperties<T>,
    pub current_transform: CameraTransform<T>,
    pub target_transform: CameraTransform<T>,
    pub smooth_enabled: bool,
    pub current_smoothness: T,
    pub maximum_smoothness: T,
}

impl<T> SmoothCamera<T>
where
    T: nalgebra::Scalar,
{
    #[inline]
    pub fn update(&mut self, delta: &CameraDelta<T>) {
        self.target_transform.update(&CameraDelta {
            time: delta.time,
            position: delta.position * self.properties.positional_velocity,
            yaw: delta.yaw * self.properties.angular_velocity,
            pitch: delta.pitch * self.properties.angular_velocity,
            fovy: delta.fovy * self.properties.zoom_velocity,
        });

        let correction = self.target_transform.correction();
        self.target_transform.correct(&correction);
        self.current_transform.correct(&correction);

        self.current_smoothness = self.target_smoothness() * 0.2 + self.current_smoothness * 0.8;

        self.current_transform = CameraTransform::interpolate(
            self.current_transform,
            self.target_transform,
            1.0 - self.current_smoothness,
        );
    }

    #[inline]
    pub fn target_smoothness(&self) -> T {
        if self.smooth_enabled {
            self.maximum_smoothness
        } else {
            0.0
        }
    }

    #[inline]
    pub fn toggle_smoothness(&mut self) {
        self.smooth_enabled = !self.smooth_enabled;
    }

    #[inline]
    pub fn current_to_camera(&self) -> Camera<T> {
        Camera {
            properties: self.properties,
            transform: self.current_transform,
        }
    }
}

#[derive(Debug)]
pub struct TransitionCamera<T>
where
    T: nalgebra::Scalar,
{
    pub start_camera: Camera<T>,
    pub current_camera: Camera<T>,
    pub progress: T,
}

pub struct TransitionCameraUpdate<'a, T>
where
    T: nalgebra::Scalar,
{
    pub delta_time: T,
    pub end_camera: &'a Camera<T>,
}

impl<T> TransitionCamera<T>
where
    T: nalgebra::Scalar,
{
    #[inline]
    pub fn start_transition(&mut self) {
        self.start_camera = self.current_camera;
        self.progress = 0.0;
    }

    #[inline]
    pub fn update(&mut self, update: TransitionCameraUpdate<T>) {
        self.progress += update.delta_time * 4.0;
        if self.progress > 1.0 {
            self.progress = 1.0;
        }

        // Bring current yaw within (-half turn, half turn) of
        // the target yaw without changing the actual angle.
        let start_yaw = self.start_camera.transform.yaw;
        let end_yaw = update.end_camera.transform.yaw;
        self.start_camera.transform.yaw = end_yaw
            + Rad((start_yaw - end_yaw + Rad::turn_div_2())
                .0
                .rem_euclid(Rad::full_turn().0))
            - Rad::turn_div_2();

        let x = self.progress;
        let t = x * x * (3.0 - 2.0 * x);

        self.current_camera = Camera::interpolate(self.start_camera, *update.end_camera, t);
    }
}
