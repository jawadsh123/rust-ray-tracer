use egui::{ColorImage, TextureHandle};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::rc::Rc;

use crate::camera::Camera;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use crate::world::{Dielectric, Lambertian, Metal, Sphere, World};

struct Config {
    max_depth: usize,
    width: usize,
    height: usize,
}

pub struct App {
    render_texture: TextureHandle,
    render_texture_img: ColorImage,

    world: World,
    camera: Camera,
    config: Config,

    rng: SmallRng,
    aa_uniform_rng: Uniform<f64>,

    sample_number: i32,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // rng
        let rng = SmallRng::from_entropy();
        let aa_uniform: Uniform<f64> = Uniform::from(0.0..1.0);

        // image
        let aspect_ratio = 16.0 / 9.0;
        let width = 480;
        let config = Config {
            width,
            height: (width as f64 / aspect_ratio) as usize,
            max_depth: 50,
        };

        // Materials
        let mat_ground = Rc::new(Lambertian {
            albedo: Color(0.8, 0.8, 0.0),
        });
        let mat_left = Rc::new(Lambertian {
            albedo: Color(0.7, 0.3, 0.2),
        });
        let mat_right = Rc::new(Metal {
            albedo: Color(0.8, 0.6, 0.2),
            fuzz: 1.0,
        });
        let mat_center = Rc::new(Dielectric {
            ior: 1.5,
            rng: SmallRng::from_entropy(),
        });

        // world
        let mut world = World { objects: vec![] };
        world.add(Box::new(Sphere {
            center: Point3(0., -100.5, -1.),
            radius: 100.,
            material: mat_ground,
        }));
        world.add(Box::new(Sphere {
            center: Point3(0., 0., -1.),
            radius: 0.5,
            material: mat_center,
        }));
        world.add(Box::new(Sphere {
            center: Point3(1., 0., -1.),
            radius: 0.5,
            material: mat_right,
        }));
        world.add(Box::new(Sphere {
            center: Point3(-1., 0., -1.),
            radius: 0.5,
            material: mat_left,
        }));

        // camera
        let origin = Point3(-2., 2., 2.);
        let camera = Camera::new(
            origin,
            (origin - Vec3(0., 0., -1.)).unit(),
            Vec3(0., 1., 0.).unit(),
            60.,
            1.,
            aspect_ratio,
        );

        // render_texture
        let img = egui::ColorImage::new(
            [config.width, config.height],
            egui::Color32::from_rgb(255, 255, 255),
        );
        let img_clone = img.clone();
        let tex_handle = cc.egui_ctx.load_texture("render", img);

        App {
            render_texture: tex_handle,
            render_texture_img: img_clone,
            world,
            camera,
            config,
            rng,
            aa_uniform_rng: aa_uniform,
            sample_number: 0,
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            render_texture,
            render_texture_img,
            world,
            camera,
            config,
            rng,
            aa_uniform_rng,
            sample_number,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            // update texture
            if sample_number < &mut 100 {
                for row in 0..config.height {
                    for col in 0..config.width {
                        let mut color = Color(0., 0., 0.);

                        let u =
                            (col as f64 + aa_uniform_rng.sample(rng)) / (config.width - 1) as f64;
                        let v = ((config.height - row - 1) as f64 + aa_uniform_rng.sample(rng))
                            / (config.height - 1) as f64;
                        let ray = camera.ray_for(u, v);
                        color += ray_color(&ray, world, config.max_depth);

                        let pixel_idx = (row * config.width + col) as usize;
                        render_texture_img.pixels[pixel_idx][0] = acc_color_channel(
                            render_texture_img.pixels[pixel_idx][0] as f64 / 255.0,
                            color.x(),
                            *sample_number,
                        );
                        render_texture_img.pixels[pixel_idx][1] = acc_color_channel(
                            render_texture_img.pixels[pixel_idx][1] as f64 / 255.0,
                            color.y(),
                            *sample_number,
                        );
                        render_texture_img.pixels[pixel_idx][2] = acc_color_channel(
                            render_texture_img.pixels[pixel_idx][2] as f64 / 255.0,
                            color.z(),
                            *sample_number,
                        );
                    }
                }
                render_texture.set(render_texture_img.clone());
                *sample_number += 1;
            }

            // draw
            ui.image(render_texture.id(), render_texture.size_vec2());
            ui.heading(format!("Samples: {}", sample_number));

            ui.horizontal(|ui| {
                ui.label("Origin:");
                let x_input = ui.add(egui::DragValue::new(&mut camera.origin.0).speed(0.2));
                let y_input = ui.add(egui::DragValue::new(&mut camera.origin.1).speed(0.2));
                let z_input = ui.add(egui::DragValue::new(&mut camera.origin.2).speed(0.2));

                if x_input.changed() || y_input.changed() || z_input.changed() {
                    camera.update();
                    *sample_number = 0;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Direction:");
                let x_input = ui.add(egui::DragValue::new(&mut camera.direction.0).speed(0.2));
                let y_input = ui.add(egui::DragValue::new(&mut camera.direction.1).speed(0.2));
                let z_input = ui.add(egui::DragValue::new(&mut camera.direction.2).speed(0.2));

                if x_input.changed() || y_input.changed() || z_input.changed() {
                    camera.update();
                    *sample_number = 0;
                }
            });

            ui.horizontal(|ui| {
                ui.label("V fov:");
                let vfov_slider = egui::Slider::new(&mut camera.vfov, 0.0..=100.0);
                if ui.add(vfov_slider).changed() {
                    camera.update();
                    *sample_number = 0;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Focal length:");
                let vfov_slider =
                    egui::Slider::new(&mut camera.focal_length, 0.0..=20.0).step_by(0.05);
                if ui.add(vfov_slider).changed() {
                    camera.update();
                    *sample_number = 0;
                }
            });
        });

        ctx.request_repaint();
    }
}

fn ray_color(ray: &Ray, world: &World, depth: usize) -> Color {
    if depth == 0 {
        return Color(0., 0., 0.);
    }
    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        match (*hit_record.material).scatter(ray, &hit_record) {
            Some(scatter_record) => {
                return scatter_record.attenuation
                    * ray_color(&scatter_record.ray, world, depth - 1)
            }
            None => return Color(0., 0., 0.),
        }
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Color(1., 1., 1.) + t * Color(0.5, 0.7, 1.)
}

// moving average + gamma correction
fn acc_color_channel(acc: f64, new_val: f64, iteration: i32) -> u8 {
    let acc_component = iteration as f64 * acc * acc;
    let new_average = (new_val + acc_component) / (iteration + 1) as f64;

    (new_average.sqrt() * 255.0) as u8
}
