// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

//! The fields under category can be retrieved by using the methods under `Category`
//! and the fields can be set for category by using the methods under `CategoryBuilder`.

/// This `Category` struct contains all the items that exist for the category field under 'Channel' and `Item`.
#[derive(Clone)]
pub struct Category {
    category: String,
    domain: Option<String>,
}


impl Category {
    /// Get the category that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category = "podcast";
    /// let category_obj = CategoryBuilder::new()
    ///     .category(category)
    ///     .finalize();
    /// assert_eq!(category.to_string(), category_obj.category());
    /// ```
    pub fn category(&self) -> String {
        self.category.clone()
    }


    /// Get the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let domain_string = "http://jupiterbroadcasting.com".to_string();
    /// let category = CategoryBuilder::new()
    ///     .domain(Some(domain_string.clone()))
    ///     .finalize();
    /// let domain_option = category.domain();
    /// assert!(domain_option.is_some());
    /// let domain = domain_option.unwrap();
    /// assert_eq!(domain_string.clone(), domain);
    /// ```
    ///
    /// ```
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new()
    ///     .domain(None)
    ///     .finalize();
    /// let domain_option = category.domain();
    /// assert!(domain_option.is_none());
    /// ```
    pub fn domain(&self) -> Option<String> {
        self.domain.clone()
    }
}


/// This `CategoryBuilder` struct creates the `Category`.
#[derive(Default)]
pub struct CategoryBuilder {
    category: String,
    domain: Option<String>,
}


impl CategoryBuilder {
    /// Construct a new `CategoryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category_builder = CategoryBuilder::new();
    /// ```
    pub fn new() -> CategoryBuilder {
        CategoryBuilder::default()
    }


    /// Set the category that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.category("Podcast");
    /// ```
    pub fn category(&mut self, category: &str) -> &mut CategoryBuilder {
        self.category = category.to_string();
        self
    }


    /// Set the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.domain(Some("http://www.example.com".to_string()));
    /// ```
    pub fn domain(&mut self, domain: Option<String>) -> &mut CategoryBuilder {
        self.domain = domain;
        self
    }


    /// Construct the `Category` from the `CategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new()
    ///         .category("Title")
    ///         .domain(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Category {
        Category {
            category: self.category.clone(),
            domain: self.domain.clone(),
        }
    }
}
