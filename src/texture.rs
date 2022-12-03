pub fn load_texture(path: &str) -> rgl::Texture {
    let texture = rgl::gen_texture();

    //Open and decode the image
    let raw_image = image::open(path).expect("Unable to open image path");

    //Convert the image into RGBA8 format
    let image = raw_image.into_rgba8();

    //Set the active texture index to 0
    rgl::active_texture(0);

    //Bind and set the texture data
    rgl::bind_texture(rgl::TexTarget::_2D, texture);
    rgl::tex_image_2d(rgl::TexTarget::_2D, 0, rgl::TexFormat::RGBA, image.width() as i32, image.height() as i32, 0, rgl::TexFormat::RGBA, &image.into_raw());

    //Set the min and mag filter
    rgl::tex_parameteri(rgl::TexTarget::_2D, rgl::TexParamName::MinFilter, rgl::TexParam::Linear);
    rgl::tex_parameteri(rgl::TexTarget::_2D, rgl::TexParamName::MagFilter, rgl::TexParam::Linear);

    //Unbind the texture
    unsafe { gl::BindTexture(gl::TEXTURE_2D, 0); }

    return texture;
}