use std::collections::HashMap;
use sdl2::hint::Hint::Default;
use crate::frame::Frame;

#[derive(Default,Debug,Clone)]
pub struct AnimationSet<'a>
{
    pub image_sheet_map: HashMap<String,String>,
    pub actions: HashMap<String, Vec<Frame<'a>>>
}

impl AnimationSet
{

    pub fn new<'a>(file_name: &str, image_cache: Option<&ImageCache>) -> Self {
        let mut out = Self {
            // TODO: AnimationSet{std::move(fileName), animationImageCache}
           ..Default::default()
        };

        let mut image_sheet_map: HashMap<String,String> = HashMap::new();
        let mut actions: HashMap<String,Vec<Frame<'a>>> = HashMap::new();
        (image_sheet_map, actions) = process_xml(file_name, image_cache);
        out.image_sheet_map = image_sheet_map.clone();
        out.actions = actions.clone();
        out
    }

    pub fn new2(image_sheet_map: HashMap<String, String>, actions: HashMap<String, Vec<Frame>>) -> Self
    {
        Self {
            image_sheet_map: image_sheet_map.clone(),
            actions: actions.clone()
        }
    }

    pub fn action_names(&mut self) -> Vec<String>
    {
        let mut out: Vec<String> = Vec::new();
        for (k, _) in self.actions.iter() {
           out.push(k.clone())
        }
        out
    }

    pub fn frames(&mut self, action_name: &str) -> Vec<Frame>
    {
        let find_res = self.actions.get(action_name).expect("frames() called on undefined action");
        find_res.clone()
    }


}



// /**
// 	 * Parses a Sprite XML Definition File.
// 	 *
// 	 * \param filePath	File path of the sprite XML definition file.
// 	 */
// 	std::tuple<ImageSheetMap, ActionsMap> processXml(const std::string& filePath, ImageCache& imageCache)
// 	{
// 		try
// 		{
// 			auto& filesystem = Utility<Filesystem>::get();
// 			const auto basePath = Filesystem::parentPath(filePath);
//
// 			Xml::XmlDocument xmlDoc;
// 			xmlDoc.parse(filesystem.readFile(filePath).c_str());
//
// 			if (xmlDoc.error())
// 			{
// 				throw std::runtime_error("Sprite file has malformed XML: Row: " + std::to_string(xmlDoc.errorRow()) + " Column: " + std::to_string(xmlDoc.errorCol()) + " : " + xmlDoc.errorDesc());
// 			}
//
// 			// Find the Sprite node.
// 			const auto* xmlRootElement = xmlDoc.firstChildElement("sprite");
// 			if (!xmlRootElement)
// 			{
// 				throw std::runtime_error("Sprite file does not contain required <sprite> tag");
// 			}
//
// 			// Get the Sprite version.
// 			const auto version = xmlRootElement->attribute("version");
// 			if (version.empty())
// 			{
// 				throw std::runtime_error("Sprite file's root element does not specify a version");
// 			}
// 			if (version != SPRITE_VERSION)
// 			{
// 				throw std::runtime_error("Sprite version mismatch. Expected: " + std::string{SPRITE_VERSION} + " Actual: " + versionString());
// 			}
//
// 			// Note:
// 			// Here instead of going through each element and calling a processing function to handle
// 			// it, we just iterate through all nodes to find sprite sheets. This allows us to define
// 			// image sheets anywhere in the sprite file.
// 			auto imageSheetMap = processImageSheets(basePath, xmlRootElement, imageCache);
// 			auto actions = processActions(imageSheetMap, xmlRootElement, imageCache);
// 			return std::tuple{std::move(imageSheetMap), std::move(actions)};
// 		}
// 		catch (const std::runtime_error& error)
// 		{
// 			throw std::runtime_error("Error parsing Sprite file: " + filePath + "\nError: " + error.what());
// 		}
// 	}
pub fn process_xml(file_path: &str, image_cache: &ImageCache) -> (HashMap<String,String>, HashMap<String,Vec<Frame>>)
{
    unimplemented!()
}

// /**
// 	 * Iterates through all elements of a Sprite XML definition looking
// 	 * for 'imagesheet' elements and processes them.
// 	 *
// 	 * \note	Since 'imagesheet' elements are processed before any other
// 	 *			element in a sprite definition, these elements can appear
// 	 *			anywhere in a Sprite XML definition.
// 	 */
// 	ImageSheetMap processImageSheets(const std::string& basePath, const Xml::XmlElement* element, ImageCache& imageCache)
// 	{
// 		ImageSheetMap imageSheetMap;
//
// 		for (const auto* node = element->firstChildElement("imagesheet"); node; node = node->nextSiblingElement("imagesheet"))
// 		{
// 			const auto dictionary = attributesToDictionary(*node);
// 			const auto id = dictionary.get("id");
// 			const auto src = dictionary.get("src");
//
// 			if (id.empty())
// 			{
// 				throw std::runtime_error("Sprite imagesheet definition has `id` of length zero: " + endTag(node->row()));
// 			}
//
// 			if (src.empty())
// 			{
// 				throw std::runtime_error("Sprite imagesheet definition has `src` of length zero: " + endTag(node->row()));
// 			}
//
// 			if (imageSheetMap.find(id) != imageSheetMap.end())
// 			{
// 				throw std::runtime_error("Sprite image sheet redefinition: id: '" + id + "' " + endTag(node->row()));
// 			}
//
// 			const auto imagePath = basePath + src;
// 			imageSheetMap.try_emplace(id, imagePath);
// 			imageCache.load(imagePath);
// 		}
//
// 		return imageSheetMap;
// 	}
pub fn process_image_sheets(base_path: &str, xml_element: (), image_cache: &ImageCache) -> Hashmap<String,String> {
    unimplemented!()
}

// /**
// 	 * Iterates through all elements of a Sprite XML definition looking
// 	 * for 'action' elements and processes them.
// 	 */
// 	ActionsMap processActions(const ImageSheetMap& imageSheetMap, const Xml::XmlElement* element, ImageCache& imageCache)
// 	{
// 		ActionsMap actions;
//
// 		for (const auto* action = element->firstChildElement("action"); action; action = action->nextSiblingElement("action"))
// 		{
// 			const auto dictionary = attributesToDictionary(*action);
// 			const auto actionName = dictionary.get("name");
//
// 			if (actionName.empty())
// 			{
// 				throw std::runtime_error("Sprite Action definition has 'name' of length zero: " + endTag(action->row()));
// 			}
// 			if (actions.find(actionName) != actions.end())
// 			{
// 				throw std::runtime_error("Sprite Action redefinition: '" + actionName + "' " + endTag(action->row()));
// 			}
//
// 			actions[actionName] = processFrames(imageSheetMap, action, imageCache);
//
// 			if (actions[actionName].empty())
// 			{
// 				throw std::runtime_error("Sprite Action contains no valid frames: " + actionName);
// 			}
// 		}
//
// 		return actions;
// 	}
pub fn process_actions(image_sheet_map: &HashMap<String,String>, element: (), image_cache: &ImageCache) -> HashMap<String,Vec<Frame>> {
    unimplemented!()
}

// /**
// 	 * Parses through all <frame> tags within an <action> tag in a Sprite Definition.
// 	 */
// 	std::vector<AnimationSet::Frame> processFrames(const ImageSheetMap& imageSheetMap, const Xml::XmlElement* element, ImageCache& imageCache)
// 	{
// 		std::vector<AnimationSet::Frame> frameList;
// 
// 		for (const auto* frame = element->firstChildElement("frame"); frame; frame = frame->nextSiblingElement("frame"))
// 		{
// 			int currentRow = frame->row();
// 
// 			const auto dictionary = attributesToDictionary(*frame);
// 			reportMissingOrUnexpected(dictionary.keys(), {"sheetid", "x", "y", "width", "height", "anchorx", "anchory"}, {"delay"});
// 
// 			const auto sheetId = dictionary.get("sheetid");
// 			const auto delay = dictionary.get<unsigned int>("delay", 0);
// 			const auto x = dictionary.get<int>("x");
// 			const auto y = dictionary.get<int>("y");
// 			const auto width = dictionary.get<int>("width");
// 			const auto height = dictionary.get<int>("height");
// 			const auto anchorx = dictionary.get<int>("anchorx");
// 			const auto anchory = dictionary.get<int>("anchory");
// 
// 			if (sheetId.empty())
// 			{
// 				throw std::runtime_error("Sprite Frame definition has 'sheetid' of length zero: " + endTag(currentRow));
// 			}
// 			const auto iterator = imageSheetMap.find(sheetId);
// 			if (iterator == imageSheetMap.end())
// 			{
// 				throw std::runtime_error("Sprite Frame definition references undefined imagesheet: '" + sheetId + "' " + endTag(currentRow));
// 			}
// 
// 			const auto& image = imageCache.load(iterator->second);
// 
// 			const auto frameRect = Rectangle<int>{{x, y}, {width, height}};
// 			const auto imageRect = Rectangle{{0, 0}, image.size()};
// 			if (!imageRect.contains(frameRect))
// 			{
// 				throw std::runtime_error("Sprite frame bounds exceeds image sheet bounds: " + endTag(currentRow));
// 			}
// 
// 			const auto anchorOffset = Vector{anchorx, anchory};
// 			frameList.push_back(AnimationSet::Frame{image, frameRect, anchorOffset, delay});
// 		}
// 
// 		return frameList;
// 	}
pub fn process_frames(image_sheet_map: &HashMap<String,String>, element: (), image_cache: &ImageCache) -> Vec<Frame>
{
    unimplemented!()
}

pub fn end_tag(row: i32) -> String
{
    format!(" (Row: {})", row)
}

