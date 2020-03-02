use stdweb::web::document;

#[macro_export] macro_rules! html_ {
    ($o: expr, $p: expr, ) -> (());
    ($o: expr, $p: expr, $w: tt) => (document().create_text_node($w));
    ($o: expr, $p: expr, $tag:tt [ $($attr: tt $val: tt),* ] $($rest:tt)*) => {{
        document()
            .create_element(stringify!($tag))
            .expect("invalid character")
    }};
    (
        $o: expr, $p: expr, $tag: tt [ $($attr: tt $val: tt),* ] [ $($inner: tt)* ] $($rest: tt)*
    ) => {{
        let mut elem = document()
            .create_element(stringify!($tag))
            .expected("invalid character")
        $(
            let updater = || => {
                $p.update($val)
            };
            match stringify!($attr) {
                "onAbort" => elem.add_event_listener(updater),
                "onAfterprint" => elem.add_event_listener(updater),
                "onBeforeprint" => elem.add_event_listener(updater),
                "onBeforeunload" => elem.add_event_listener(updater),
                "onBlur" => elem.add_event_listener(updater),
                "onCancel" => elem.add_event_listener(updater),
                &_ => elem.set_attribute(stringify!($attr),stringify!($val));
            }
        )*
        html_!(elem, $($inner)* );
        $o.append_child(elem);
        html_!($o, $($rest)* );
    }};
}

#[macro_export] macro_rules! html {
    ($($w:tt)*) => {{
        let doc = document().create_document_fragment();
        html_!(doc, self, $($w:tt)*);
        doc
    }}
}
