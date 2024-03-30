use std::fmt::Display;

pub trait UtilStringGenerator {
    fn space(self) -> Self;
    fn tab(self) -> Self;
    fn tabs(self, amount: usize) -> Self;

    fn new_line(self) -> Self;
    fn new_lines(self, amount: usize) -> Self;

    fn open_bracket(self) -> Self;
    fn close_bracket(self) -> Self;

    fn comma(self) -> Self;

    fn export(self) -> Self;
    fn const_(self, name: impl AsRef<str>, ty: impl AsRef<str>, value: impl AsRef<str>) -> Self;

    fn finish(self) -> String;
}

impl UtilStringGenerator for String {
    fn space(mut self) -> Self {
        self.push(' ');
        self
    }

    fn tab(mut self) -> Self {
        self.push('\t');
        self
    }

    fn tabs(mut self, amount: usize) -> Self {
        self.push_str(&"\t".repeat(amount));
        self
    }

    fn new_line(mut self) -> Self {
        self.push('\n');
        self
    }

    fn new_lines(mut self, amount: usize) -> Self {
        self.push_str(&"\n".repeat(amount));
        self
    }

    fn open_bracket(mut self) -> Self {
        self.push('{');
        self
    }

    fn close_bracket(mut self) -> Self {
        self.push('}');
        self
    }

    fn comma(mut self) -> Self {
        self.push(',');
        self
    }

    fn export(mut self) -> Self {
        self.push_str("export");
        self.space()
    }

    fn const_(
        mut self,
        name: impl AsRef<str>,
        ty: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Self {
        self.push_str("const ");
        self.push_str(name.as_ref());
        self.push(':');
        self = self.space();
        self.push_str(ty.as_ref());
        self = self.space();
        self.push('=');
        self = self.space();
        self.push_str(value.as_ref());
        self.push(';');

        self
    }

    fn finish(self) -> String {
        self
    }
}

pub trait TypeStringGenerator: UtilStringGenerator {
    fn class(self) -> impl ClassStringGenerator;
    fn enum_(self) -> impl EnumStringGenerator;
}

impl TypeStringGenerator for String {
    fn class(mut self) -> impl ClassStringGenerator {
        self.push_str("class");
        self.space()
    }

    fn enum_(mut self) -> impl EnumStringGenerator {
        self.push_str("enum");
        self.space()
    }
}

pub trait ClassStringGenerator: UtilStringGenerator {}

impl ClassStringGenerator for String {}

pub trait EnumStringGenerator: UtilStringGenerator {
    fn name(self, name: impl AsRef<str>) -> Self;
    fn variants(self, variants: &[String]) -> Self;
    fn variant(self, variant: impl AsRef<str>) -> Self;
    fn default(
        self,
        enum_name: impl AsRef<str>,
        name: impl AsRef<str>,
        variant: impl Display,
    ) -> Self;
}

impl EnumStringGenerator for String {
    fn name(mut self, name: impl AsRef<str>) -> Self {
        self.push_str(name.as_ref());
        self.space()
    }

    fn variants(self, variants: &[String]) -> Self {
        variants
            .iter()
            .fold(self.open_bracket().new_line(), |res, variant| {
                res.tab().variant(variant).new_line()
            })
            .close_bracket()
    }

    fn variant(mut self, variant: impl AsRef<str>) -> Self {
        self.push_str(variant.as_ref());
        self.comma()
    }

    fn default(
        self,
        enum_name: impl AsRef<str>,
        name: impl AsRef<str>,
        variant: impl Display,
    ) -> Self {
        let enum_name = enum_name.as_ref();

        self.export()
            .const_(name, enum_name, format!("{enum_name}.{variant}"))
    }
}
