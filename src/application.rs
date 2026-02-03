use std::ops::{Add, Sub};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime};

use chrono::{DateTime, Local};
use iced::Alignment;
use iced::time::{every, seconds};
use iced::widget::{button, column, row, text, text_input};
use iced::{Element, Subscription, Task};

use crate::utils::{make_secs, regulate_24, regulate_60};

pub const WINDOW_WIDTH: f32 = 720.0;
pub const WINDOW_HEIGHT: f32 = 320.0;

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    HoursEdited(String),
    MinutesEdited(String),
    SecondsEdited(String),
    StartTimer,
    StopTimer,
    ResetTimer,
}

pub struct Application {
    hours: String,
    minutes: String,
    seconds: String,
    countdown: bool,
    start_time: DateTime<Local>,
    finish_time: DateTime<Local>,
}

impl Application {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                hours: "00".to_string(),
                minutes: "00".to_string(),
                seconds: "00".to_string(),
                countdown: false,
                start_time: SystemTime::now().into(),
                finish_time: SystemTime::now().sub(Duration::from_secs(3600)).into(),
            },
            Task::none(),
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        let current_datetime: DateTime<Local> = SystemTime::now().into();
        let time_str = format!(
            "Current time: {}",
            current_datetime.format("%H:%M:%S").to_string()
        );
        let shutdown_str = format!(
            "Shutdown at: {}",
            if self.countdown {
                self.finish_time.format("%H:%M:%S").to_string()
            } else {
                "Not set".to_string()
            }
        );

        let hours_input = column![
            text("Hours"),
            text_input("00", &self.hours)
                .on_input(Message::HoursEdited)
                .width(35)
        ]
        .align_x(Alignment::Center)
        .spacing(10);
        let minutes_input = column![
            text("Minutes"),
            text_input("00", &self.minutes)
                .on_input(Message::MinutesEdited)
                .width(35)
        ]
        .align_x(Alignment::Center)
        .spacing(10);
        let seconds_input = column![
            text("Seconds"),
            text_input("00", &self.seconds)
                .on_input(Message::SecondsEdited)
                .width(35)
        ]
        .align_x(Alignment::Center)
        .spacing(10);

        let inputs_row = row![hours_input, minutes_input, seconds_input].spacing(25);

        let start_button =
            button(text("Start").align_x(Alignment::Center)).on_press(Message::StartTimer);
        let stop_button = button(text("Stop").align_x(Alignment::Center))
            .on_press(Message::StopTimer)
            .style(button::danger);

        let action_button = (if self.countdown {
            stop_button
        } else {
            start_button
        })
        .width(100);
        let reset_button = button(text("Reset").align_x(Alignment::Center))
            .on_press(Message::ResetTimer)
            .width(100)
            .style(button::warning);

        column![
            text("Shutdown Timer").size(48),
            row![text(time_str), text(shutdown_str)].spacing(24),
            row![].height(20),
            inputs_row,
            row![].height(60),
            row![action_button, reset_button].spacing(24)
        ]
        .align_x(Alignment::Center)
        .spacing(12)
        .width(WINDOW_WIDTH)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick(_now) => {
                let current_datetime: DateTime<Local> = SystemTime::now().into();
                let currant_timestamp = current_datetime.timestamp();
                let finish_timestamp = self.finish_time.timestamp();
                if self.countdown && currant_timestamp >= finish_timestamp {
                    println!("Shutting down");
                    self.shutdown_command();
                }
            }
            Message::HoursEdited(changed_hours) => {
                self.hours = regulate_24(changed_hours);
            }
            Message::MinutesEdited(changed_minutes) => {
                self.minutes = regulate_60(changed_minutes);
            }
            Message::SecondsEdited(changed_seconds) => {
                self.seconds = regulate_60(changed_seconds);
            }
            Message::StartTimer => {
                let elapsed_secs = make_secs(
                    self.hours.clone(),
                    self.minutes.clone(),
                    self.seconds.clone(),
                );
                if elapsed_secs > 0 {
                    self.start_time = SystemTime::now().into();
                    self.finish_time = self
                        .start_time
                        .clone()
                        .add(Duration::from_secs(elapsed_secs));
                    self.countdown = true;
                }
            }
            Message::StopTimer => {
                self.finish_time = self.start_time.clone().sub(Duration::from_secs(3600));
                self.countdown = false;
            }
            Message::ResetTimer => {
                self.hours = "00".to_string();
                self.minutes = "00".to_string();
                self.seconds = "00".to_string();
                self.start_time = SystemTime::now().into();
                self.finish_time = self.start_time.clone().sub(Duration::from_secs(3600));
                self.countdown = false;
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        every(seconds(1)).map(Message::Tick)
    }

    fn shutdown_command(&self) {
        let output = if cfg!(target_os = "windows") {
            Command::new("shutdown")
                .args(&["/s", "/f", "/t", "0"])
                .spawn()
                .expect("Failed to execute process");
        } else {
            Command::new("systemctl")
                .arg("poweroff")
                .spawn()
                .expect("Failed to execute process");
        };
        println!("{:?}", output);
    }
}
