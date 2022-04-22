pub mod async_return {
	use std::sync::mpsc::{self, Receiver, RecvError, SendError, Sender, TryRecvError};

	pub struct Returns<T> {
		recv: Receiver<T>,
	}

	impl<T> Returns<T> {
		pub fn new() -> (Self, Returner<T>) {
			let (send, recv) = mpsc::channel();
			(Self { recv }, Returner { send })
		}
		pub fn get(self) -> Result<T, RecvError> {
			self.recv.recv()
		}
		pub fn try_get(&mut self) -> Result<T, TryRecvError> {
			self.recv.try_recv()
		}
	}

	pub struct Returner<T> {
		send: Sender<T>,
	}

	impl<T> Returner<T> {
		pub fn ret(self, data: T) -> Result<(), SendError<T>> {
			self.send.send(data)
		}
	}

	mod tests {

		#[test]
		fn return_test() {
			use super::Returns;
			use std::{thread, time::Duration};
			let (returns, returner) = Returns::new();
			thread::spawn(move || {
				thread::sleep(Duration::from_millis(1000));
				returner.ret(69).unwrap();
			});
			for i in 0..10 {
				println!("{}", i);
				thread::sleep(Duration::from_millis(100));
			}
			assert_eq!(returns.get().unwrap(), 69);
		}
	}
}

pub mod tex {
	use std::path::Path;

	use glium::{
		backend::Facade,
		texture::{
			BindlessTexturesNotSupportedError, RawImage2d, ResidentTexture, SrgbTexture2d,
			TextureCreationError,
		},
	};
	use image::{ImageBuffer, ImageError, Rgba};

	pub fn rgba8_srgb2d<P: AsRef<Path>, F: Facade + ?Sized>(
		path: P,
		facade: &F,
	) -> Result<(ImageBuffer<Rgba<u8>, Vec<u8>>, ResidentTexture), TextureLoadError> {
		let img = image::io::Reader::open(path)?.decode()?.into_rgba8();
		let texture: SrgbTexture2d = SrgbTexture2d::new(
			facade,
			RawImage2d::from_raw_rgba_reversed(img.as_raw(), img.dimensions()),
		)?;
		let texture = texture.resident()?;
		Ok((img, texture))
	}

	#[derive(Debug)]
	pub enum TextureLoadError {
		ImageError(ImageError),
		TextureCreationError(TextureCreationError),
		BindlessTexturesNotSupportedError(BindlessTexturesNotSupportedError),
		IoError(std::io::Error),
	}

	impl From<ImageError> for TextureLoadError {
		fn from(e: ImageError) -> Self {
			Self::ImageError(e)
		}
	}
	impl From<TextureCreationError> for TextureLoadError {
		fn from(e: TextureCreationError) -> Self {
			Self::TextureCreationError(e)
		}
	}
	impl From<BindlessTexturesNotSupportedError> for TextureLoadError {
		fn from(e: BindlessTexturesNotSupportedError) -> Self {
			Self::BindlessTexturesNotSupportedError(e)
		}
	}
	impl From<std::io::Error> for TextureLoadError {
		fn from(e: std::io::Error) -> Self {
			Self::IoError(e)
		}
	}
}
