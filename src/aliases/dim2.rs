use nalgebra::mat::{Mat2, Mat1};
use nalgebra::vec::{Vec2, Vec1};
use nalgebra::adaptors::transform::Transform;
use nalgebra::adaptors::rotmat::Rotmat;
use ncollide::geom::ball::Ball;
use ncollide::geom::plane::Plane;
use ncollide::geom::box::Box;
use integration::body_force_generator::BodyForceGenerator;
use integration::rigid_body_integrator::RigidBodySmpEulerIntegrator;
use detection::collision::bodies_bodies::{LBVBodiesBodies, Constraint};
use resolution::constraint::accumulated_impulse_solver::AccumulatedImpulseSolver;
use object::implicit_geom::DefaultGeom;
use object::rigid_body::RigidBody;
use object::body::Body;
use world::world::World;

type LV<N> = Vec2<N>;
type AV<N> = Vec1<N>;
type II<N> = Mat1<N>;
type M<N>  = Transform<Rotmat<Mat2<N>>, Vec2<N>>;

// fancier names
pub type Transform2d<N>       = M<N>;
pub type LinearVelocity2d<N>  = LV<N>;
pub type AngularVelocity2d<N> = AV<N>;
pub type InertiaTensor2d<N>   = II<N>;

pub type Ball2d<N>  = Ball<N, Vec2<N>>;
pub type Box2d<N>   = Box<N, Vec2<N>>;
pub type Plane2d<N> = Plane<Vec2<N>>;
pub type Geom2d<N>  = DefaultGeom<N, LV<N>, M<N>, II<N>>;

pub type ForceGenerator2d<N> = BodyForceGenerator<N, LV<N>, AV<N>, M<N>, II<N>>;
pub type RigidBodyIntegrator2d<N> = RigidBodySmpEulerIntegrator<N, LV<N>, AV<N>, M<N>, II<N>>;

pub type CollisionDetector2d<N> = LBVBodiesBodies<N, LV<N>, AV<N>, M<N>, II<N>>;

pub type ContactSolver2d<N> = AccumulatedImpulseSolver<N, LV<N>, AV<N>, M<N>, II<N>>;

pub type Constraint2d<N> = Constraint<N, LV<N>, AV<N>, M<N>, II<N>>;
pub type RigidBody2d<N> = RigidBody<N, LV<N>, AV<N>, M<N>, II<N>>; 
pub type Body2d<N> = Body<N, LV<N>, AV<N>, M<N>, II<N>>; 
pub type World2d<N> = World<N, Body2d<N>, Constraint2d<N>>;