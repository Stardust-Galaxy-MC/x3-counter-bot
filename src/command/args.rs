use serenity::all::{CommandOption, CommandOptionType, CreateCommandOption};

pub trait IntoCommandArg: PartialEq<CommandOption> {
	fn name(&self) -> &str;
	fn to_arg(&self) -> CreateCommandOption;
}

#[derive(Debug, Clone)]
pub struct BaseArg {
	pub name: &'static str,
	pub description: &'static str,
	pub required: bool,
}

impl PartialEq<CommandOption> for BaseArg {
	fn eq(&self, other: &CommandOption) -> bool {
		other.required == self.required
			&& other.name == self.name
			&& other.description == self.description
	}
}

impl BaseArg {
	fn to_arg(&self, kind: CommandOptionType) -> CreateCommandOption {
		CreateCommandOption::new(kind, self.name, self.description).required(self.required)
	}
}

#[derive(Debug, Clone)]
pub struct IntArg {
	pub base: BaseArg,
	pub min: Option<u64>,
	pub max: Option<u64>,
}

impl PartialEq<CommandOption> for IntArg {
	fn eq(&self, other: &CommandOption) -> bool {
		other.kind == CommandOptionType::Integer
			&& other.min_value.as_ref().and_then(|v| v.as_u64()) == self.min
			&& other.max_value.as_ref().and_then(|v| v.as_u64()) == self.max
			&& self.base.eq(other)
	}
}

impl IntoCommandArg for IntArg {
	fn name(&self) -> &str {
		self.base.name
	}

	fn to_arg(&self) -> CreateCommandOption {
		let mut option = self.base.to_arg(CommandOptionType::Integer);
		if let Some(min) = self.min {
			option = option.min_int_value(min);
		}
		if let Some(max) = self.max {
			option = option.max_int_value(max);
		}
		option
	}
}

#[derive(Debug, Clone)]
pub struct UserArg {
	pub base: BaseArg,
}

impl PartialEq<CommandOption> for UserArg {
	fn eq(&self, other: &CommandOption) -> bool {
		other.kind == CommandOptionType::User && self.base.eq(other)
	}
}

impl IntoCommandArg for UserArg {
	fn name(&self) -> &str {
		self.base.name
	}

	fn to_arg(&self) -> CreateCommandOption {
		self.base.to_arg(CommandOptionType::User)
	}
}

#[derive(Debug, Clone)]
pub struct StringArg {
	pub base: BaseArg,
	pub choices: &'static str,
}

impl StringArg {
	fn gen_choices(&self) -> Vec<&str> {
		match self.choices {
			"" => Vec::new(),
			other => other.split('\n').filter(|s| !s.is_empty()).collect(),
		}
	}
}

impl PartialEq<CommandOption> for StringArg {
	fn eq(&self, other: &CommandOption) -> bool {
		if !(other.kind == CommandOptionType::String && self.base.eq(other)) {
			return false;
		}

		let mut new_choices: Vec<&str> = self.gen_choices();
		new_choices.sort();
		let mut old_choices: Vec<&str> = other.choices.iter().map(|e| e.name.as_str()).collect();
		old_choices.sort();

		if old_choices.len() != new_choices.iter().len() {
			return false;
		}

		old_choices
			.into_iter()
			.zip(new_choices)
			.all(|(l, r)| l == r)
	}
}

impl IntoCommandArg for StringArg {
	fn name(&self) -> &str {
		self.base.name
	}

	fn to_arg(&self) -> CreateCommandOption {
		let mut option = self.base.to_arg(CommandOptionType::String);
		for choice in self.gen_choices() {
			option = option.add_string_choice(choice, choice);
		}
		option
	}
}
