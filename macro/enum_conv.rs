macro_rules! enum_conv {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident $(= $value:expr)?),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant $(= $value)?),*
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($variant) => Ok($name::$variant),)*
                    _ => Err(format!("unknown variant: {}", s)),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, stringify!($variant)),)*
                }
            }
        }
    };
}
