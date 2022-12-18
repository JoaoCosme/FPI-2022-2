mod ui{
    use egui_extras::RetainedImage;
    pub struct App{
        image: RetainedImage,
        tint : egui::Color32
    }

    impl Default for App{
        fn default() -> Self {
            Self { image, 
            tint: egui::Color32::from_rgb(255, 0, 255)
                        }
        }
    }
}