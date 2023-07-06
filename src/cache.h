#pragma once

#include "font.h"
#include "image.h"
#include "music.h"
#include "resourcecache.h"

#include <memory>


inline NAS2D::ResourceCache<NAS2D::Font, std::string, unsigned int> fontCache;
inline NAS2D::ResourceCache<NAS2D::Image, std::string> imageCache;

inline std::unique_ptr<NAS2D::Music> trackMars;
