// TODO refine: integrate into style.rs or make button component functions

pub enum Style {
    Basic,
    Color1,
}

impl Style {
    pub fn as_str(&self) -> &'static str {
        match self {
            Style::Basic => "p-4 rounded-lg bg-blue-400 hover:bg-blue-500 font-bold text-white shadow-lg shadow-blue-200 transition ease-in-out duration-200 translate-10",
            Style::Color1 => "flex max-w-sm w-full bg-gradient-to-r from-indigo-500 via-pink-500 to-yellow-500 hover:from-indigo-600 hover:via-pink-600 hover:to-red-600 focus:outline-none text-white text-2xl uppercase font-bold shadow-md mx-auto p-5"
        }
    }
}
