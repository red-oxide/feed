// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! All the structs for channels.


pub mod category;
pub mod category_builder;
pub mod channel;
pub mod channel_builder;
pub mod cloud;
pub mod cloud_builder;
pub mod enclosure;
pub mod enclosure_builder;
pub mod guid;
pub mod guid_builder;
pub mod image;
pub mod image_builder;
pub mod item;
pub mod item_builder;
pub mod source;
pub mod source_builder;
pub mod text_input;
pub mod text_input_builder;

pub mod itunes;

use self::itunes::{ITunesChannelExtension, ITunesItemExtension};
use chrono::*;
use enums::{CloudProtocol, Day};
use mime::Mime;
use url::Url;


/// This `Category` struct contains all the items that exist for the category
/// field under 'Channel' and `Item`.
#[derive(Clone)]
pub struct Category
{
    name: String,
    domain: Option<Url>,
}


/// This `CategoryBuilder` struct creates the `Category`.
#[derive(Clone, Default)]
pub struct CategoryBuilder
{
    name: String,
    domain: Option<String>,
}


/// This `Channel` struct contains all the items that exist for the `feed`.
#[derive(Clone)]
pub struct Channel
{
    title: String,
    link: Url,
    description: String,
    language: Option<String>,
    copyright: Option<String>,
    managing_editor: Option<String>,
    web_master: Option<String>,
    pub_date: Option<DateTime<FixedOffset>>,
    last_build_date: Option<DateTime<FixedOffset>>,
    categories: Option<Vec<Category>>,
    generator: Option<String>,
    docs: Option<Url>,
    cloud: Option<Cloud>,
    ttl: Option<i64>,
    image: Option<Image>,
    rating: Option<String>,
    text_input: Option<TextInput>,
    skip_hours: Option<Vec<i64>>,
    skip_days: Option<Vec<Day>>,
    items: Option<Vec<Item>>,
    itunes_ext: Option<ITunesChannelExtension>,
}


/// This `ChannelBuilder` struct creates the `Channel`.
#[derive(Clone, Default)]
pub struct ChannelBuilder
{
    title: String,
    link: String,
    description: String,
    language: Option<String>,
    copyright: Option<String>,
    managing_editor: Option<String>,
    web_master: Option<String>,
    pub_date: Option<String>,
    last_build_date: Option<String>,
    categories: Option<Vec<Category>>,
    generator: Option<String>,
    docs: Option<String>,
    cloud: Option<Cloud>,
    ttl: Option<i64>,
    image: Option<Image>,
    rating: Option<String>,
    text_input: Option<TextInput>,
    skip_hours: Option<Vec<i64>>,
    skip_days: Option<Vec<String>>,
    items: Option<Vec<Item>>,
    itunes_ext: Option<ITunesChannelExtension>,
}


/// This `Cloud` struct contains all the items that exist for the cloud field
/// under 'Channel'.
#[derive(Clone)]
pub struct Cloud
{
    domain: Url,
    port: i64,
    path: String,
    register_procedure: String,
    protocol: CloudProtocol,
}


/// This `CloudBuilder` struct creates the `Cloud`.
#[derive(Clone, Default)]
pub struct CloudBuilder
{
    domain: String,
    port: i64,
    path: String,
    register_procedure: String,
    protocol: String,
}


/// This `Enclosure` struct contains all the items that exist for the enclosure
/// field under 'Item'.
#[derive(Clone)]
pub struct Enclosure
{
    url: Url,
    length: i64,
    mime_type: Mime,
}


/// This `EnclosureBuilder` struct creates the `Enclosure`.
#[derive(Clone, Default)]
pub struct EnclosureBuilder
{
    url: String,
    length: i64,
    mime_type: String,
}


/// This `Guid` struct contains all the items that exist for the guid field
/// under 'Item'.
#[derive(Clone)]
pub struct Guid
{
    permalink: bool,
    value: String,
}


/// This `GuidBuilder` struct creates the `Guid`.
#[derive(Clone, Default)]
pub struct GuidBuilder
{
    permalink: Option<bool>,
    value: String,
}


/// This `Image` struct contains all the items that exist for the image field
/// under 'Channel'.
#[derive(Clone)]
pub struct Image
{
    url: Url,
    title: String,
    link: Url,
    width: i64,
    height: i64,
    description: Option<String>,
}


/// This `ImageBuilder` struct creates the `Image`.
#[derive(Clone, Default)]
pub struct ImageBuilder
{
    url: String,
    title: String,
    link: String,
    width: Option<i64>,
    height: Option<i64>,
    description: Option<String>,
}


/// This `Item` struct contains all the items that exist for the item field
/// under 'Channel'.
#[derive(Clone)]
pub struct Item
{
    title: Option<String>,
    link: Option<Url>,
    description: Option<String>,
    author: Option<String>,
    categories: Option<Vec<Category>>,
    comments: Option<Url>,
    enclosure: Option<Enclosure>,
    guid: Option<Guid>,
    pub_date: Option<DateTime<FixedOffset>>,
    source: Option<Source>,
    itunes_ext: Option<ITunesItemExtension>,
}


/// This `ItemBuilder` struct creates the `Item`.
#[derive(Clone, Default)]
pub struct ItemBuilder
{
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    author: Option<String>,
    categories: Option<Vec<Category>>,
    comments: Option<String>,
    enclosure: Option<Enclosure>,
    guid: Option<Guid>,
    pub_date: Option<String>,
    source: Option<Source>,
    itunes_ext: Option<ITunesItemExtension>,
}


/// This `Source` struct contains all the items that exist for the source field
/// under 'Item'.
#[derive(Clone)]
pub struct Source
{
    url: Url,
    title: Option<String>,
}


/// This `SourceBuilder` struct creates the `Source`.
#[derive(Clone, Default)]
pub struct SourceBuilder
{
    url: String,
    title: Option<String>,
}


/// This `TextInput` struct contains all the items that exist for the text input
/// field under 'Channel'.
#[derive(Clone)]
pub struct TextInput
{
    title: String,
    description: String,
    name: String,
    link: Url,
}


/// This `TextInputBuilder` struct creates the `TextInput`.
#[derive(Clone, Default)]
pub struct TextInputBuilder
{
    title: String,
    description: String,
    name: String,
    link: String,
}
