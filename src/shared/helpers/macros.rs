#[macro_export]
macro_rules! selectable_values {
    ($ui:expr, $target:expr, $( $variant:expr => $label:expr ),+ $(,)?) => {
        $(
            $ui.selectable_value($target, $variant, $label);
        )+
    };
}

#[macro_export]
macro_rules! try_or_return {
    ($self:expr, $ui:expr, $result:expr) => {
        match $result {
            Ok(v) => v,
            Err(err) => {
                $self.err = err.to_string();
                log::error!("{}", err);
                $self.display_error($ui.ctx(), file!(), line!());
                return;
            }
        }
    };
}
