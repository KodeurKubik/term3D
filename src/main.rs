mod data;
mod draw_line;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::Stylize;
use crossterm::{cursor, event, execute, queue, style, terminal};
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::data::{EDGE, VERTEX};
use crate::draw_line::draw_line;

const FOCAL_LENGTH: i16 = 100;

#[allow(dead_code)]
#[derive(PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

fn rotate_around(point: (i16, i16, i16), theta: f32, axis: Axis) -> (i16, i16, i16) {
    let theta_rad = theta.to_radians();
    let cos_theta = theta_rad.cos();
    let sin_theta = theta_rad.sin();
    let (x, y, z) = point;

    match axis {
        Axis::X => {
            let new_y = (y as f32 * cos_theta - z as f32 * sin_theta).round() as i16;
            let new_z = (y as f32 * sin_theta + z as f32 * cos_theta).round() as i16;
            (x, new_y, new_z)
        }
        Axis::Y => {
            let new_x = (x as f32 * cos_theta + z as f32 * sin_theta).round() as i16;
            let new_z = (-x as f32 * sin_theta + z as f32 * cos_theta).round() as i16;
            (new_x, y, new_z)
        }
        Axis::Z => {
            let new_x = (x as f32 * cos_theta - y as f32 * sin_theta).round() as i16;
            let new_y = (x as f32 * sin_theta + y as f32 * cos_theta).round() as i16;
            (new_x, new_y, z)
        }
    }
}

fn project_vertex(vertex: (i16, i16, i16)) -> Option<(u16, u16)> {
    let camera_distance = 350;

    let projected_x = (FOCAL_LENGTH as i32 * vertex.0 as i32)
        / (vertex.2 as i32 + FOCAL_LENGTH as i32 + camera_distance);
    let projected_y = (FOCAL_LENGTH as i32 * vertex.1 as i32)
        / (vertex.2 as i32 + FOCAL_LENGTH as i32 + camera_distance);

    let screen_x = projected_x + 72;
    let screen_y = projected_y + 72;

    return Some((screen_x as u16, screen_y as u16));
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide)?;

    let mut angle: f32 = 0f32;
    let target_fps = 60;
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);

    loop {
        let frame_start = Instant::now();

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                if modifiers.contains(event::KeyModifiers::CONTROL) {
                    break; // Exit on Ctrl+C
                }
            }
        }

        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
        let projected_vertex: Vec<Option<(u16, u16)>> = VERTEX
            .iter()
            .map(|&(x, y, z)| {
                project_vertex(rotate_around(
                    rotate_around((x, y, z), angle, Axis::Y),
                    40f32,
                    Axis::X,
                ))
            })
            .collect();

        for edge in EDGE {
            if let (Some(start), Some(end)) = (projected_vertex[edge.0], projected_vertex[edge.1]) {
                let pixels = draw_line(start, end)?;
                for px in pixels {
                    queue!(stdout, cursor::MoveTo(px.0, px.1), style::Print('#'))?;
                }
            }
        }

        for vertex in projected_vertex {
            if let Some(vert) = vertex {
                queue!(
                    stdout,
                    cursor::MoveTo(vert.0, vert.1),
                    style::PrintStyledContent('#'.magenta())
                )?;
            }
        }

        stdout.flush()?;
        angle += 1f32;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    execute!(stdout, cursor::MoveTo(0, 100), cursor::Show)?;
    println!();
    Ok(())
}
