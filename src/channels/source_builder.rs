// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields can be set for source by using the methods under `SourceBuilder`.


use channels::{Source, SourceBuilder};


impl SourceBuilder {
    /// Construct a new `SourceBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let source_builder = SourceBuilder::new();
    /// ```
    pub fn new() -> SourceBuilder {
        SourceBuilder::default()
    }


    /// Set the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.url("http://www.example.com/source");
    /// ```
    pub fn url(&mut self, url: &str) -> &mut SourceBuilder {
        self.url = url.to_owned();
        self
    }


    /// Set the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.title(Some("Test".to_owned()));
    /// ```
    pub fn title(&mut self, title: Option<String>) -> &mut SourceBuilder {
        self.title = title;
        self
    }


    /// Construct the `Source` from the `SourceBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let source = SourceBuilder::new()
    ///         .url("http://www.example.com/source")
    ///         .title(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Source {
        Source {
            url: self.url.clone(),
            title: self.title.clone(),
        }
    }
}
