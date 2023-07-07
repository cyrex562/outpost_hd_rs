// #pragma once

// #include"wrapper.h"
// #include"crimerateupdate.h"
// #include"crimeexecution.h"
// #include"structuretracker.h"

// #include"planet.h"

// #include"numbers.h"
// #include"uiconstants.h"

// #include"common.h"
// #include"storableresources.h"
// #include"robotpool.h"
// #include"populationpool.h"
// #include"population.h"

// #include"researchtracker.h"
// #include"technologycatalog.h"

// #include"robot.h"
// #include"structure.h"

// #include"notificationarea.h"
// #include"notificationwindow.h"
// #include"diggerdirection.h"
// #include"factoryproduction.h"
// #include"fileio.h"
// #include"gameoverdialog.h"
// #include"gameoptionsdialog.h"
// #include"icongrid.h"
// #include"majoreventannouncement.h"
// #include"mineoperationswindow.h"
// #include"populationpanel.h"
// #include"resourcebreakdownpanel.h"
// #include"robotinspector.h"
// #include"structureinspector.h"
// #include"tileinspector.h"
// #include"warehouseinspector.h"
// #include"resourceinfobar.h"
// #include"robotdeploymentsummary.h"
// #include"minimap.h"
// #include"detailmap.h"
// #include"navcontrol.h"
// #include"cheatmenu.h"

// #include"windowstack.h"
// #include"tooltip.h"

// #include"signal.h"
// #include"point.h"
// #include"rectangle.h"
// #include"fade.h"

// #include<string>
// #include<memory>
// #include<map>


namespace NAS2D
{
	namespace Xml
	{
		class XmlElement;
	}
}

namespace micropather
{
	class MicroPather;
}

struct MapCoordinate;
class Tile;
class TileMap;
class MapView;
class MainReportsUiState;


enum PointerType
{
	POINTER_NORMAL,
	POINTER_PLACE_TILE
};


enum class InsertMode
{
	None,
	Robot,
	Tube,
	Structure
};

using RobotTileTable = std::map<Robot*, Tile*>;


extern const NAS2D::Font* MAIN_FONT;


class MapViewState : public Wrapper
{
public:
	enum class PopulationLevel
	{
		Small = 1,
		Large = 2
	};

public:
	using QuitSignal = NAS2D::Signal<>;
	using ReportsUiSignal = NAS2D::Signal<>;
	using MapChangedSignal = NAS2D::Signal<>;

public:
	MapViewState(MainReportsUiState&, const std::string& savegame);
	MapViewState(MainReportsUiState&, const Planet::Attributes& planetAttributes, Difficulty selectedDifficulty);
	~MapViewState() override;

	void setPopulationLevel(PopulationLevel popLevel);

	ReportsUiSignal::Source& showReporstUi() { return mReportsUiSignal; }
	QuitSignal::Source& quit() { return mQuitSignal; }
	MapChangedSignal::Source& mapChanged() { return mMapChangedSignal; }

	void focusOnStructure(Structure* s);

	Difficulty difficulty() { return mDifficulty; }
	void difficulty(Difficulty difficulty);

	bool hasGameEnded();

protected:
	void initialize() override;
	State* update() override;

private:
	void _deactivate() override;
	void _activate() override;

	// EVENT HANDLERS
	void onActivate(bool newActiveValue);
	void onKeyDown(NAS2D::EventHandler::KeyCode key, NAS2D::EventHandler::KeyModifier mod, bool repeat);
	void onMouseDown(NAS2D::EventHandler::MouseButton button, NAS2D::Point<int> position);
	void onMouseDoubleClick(NAS2D::EventHandler::MouseButton button, NAS2D::Point<int> position);
	void onMouseUp(NAS2D::EventHandler::MouseButton button, NAS2D::Point<int> position);
	void onMouseMove(NAS2D::Point<int> position, NAS2D::Vector<int> relative);
	void onMouseWheel(NAS2D::Vector<int> changeAmount);
	void onWindowResized(NAS2D::Vector<int> newSize);

	void onInspect(const MapCoordinate& tilePosition, bool inspectModifier);
	void onInspectStructure(Structure& structure, bool inspectModifier);
	void onInspectRobot(Robot& robot);
	void onInspectTile(Tile& tile);

	void onClickMap();

	void onSystemMenu();

	// ROBOT EVENT HANDLERS
	void onDozerTaskComplete(Robot* robot);
	void onDiggerTaskComplete(Robot* robot);
	void onMinerTaskComplete(Robot* robot);

	// DRAWING FUNCTIONS
	void drawUI();
	void drawSystemButton() const;

	// INSERT OBJECT HANDLING
	void onDeployCargoLander();
	void onDeployColonistLander();
	void onDeploySeedLander(NAS2D::Point<int> point);
	void insertSeedLander(NAS2D::Point<int> point);
	void insertTube(ConnectorDir dir, int depth, Tile& tile);

	void placeTubes(Tile& tile);
	void placeStructure(Tile& tile);
	void placeRobot(Tile& tile);

	void placeRobodozer(Tile&);
	void placeRobodigger(Tile&);
	void placeRobominer(Tile&);

	Robot& addRobot(Robot::Type type);

	void setStructureID(StructureID type, InsertMode mode);

	// MISCELLANEOUS UTILITY FUNCTIONS
	void updateFood();
	void transferFoodToCommandCenter();

	void updateCommRangeOverlay();
	void updatePoliceOverlay();
	void resetPoliceOverlays();
	void updateConnectedness();
	void changeViewDepth(int);

	void pullRobotFromFactory(ProductType pt, Factory& factory);
	void onFactoryProductionComplete(Factory& factory);
	void onCheatCodeEntry(const std::string& cheatCode);

	void onMineFacilityExtend(MineFacility* mf);

	void updatePlayerResources();
	void updateResearch();

	// TURN LOGIC
	void checkColonyShip();
	void checkWarehouseCapacity();
	void nextTurn();
	void updatePopulation();
	void updateCommercial();
	void updateMaintenance();
	void updateMorale();
	void updateResidentialCapacity();
	void updateBiowasteRecycling();
	void updateResources();
	void updateRoads();
	void updateRobots();

	void findMineRoutes();
	void transportOreFromMines();
	void transportResourcesToStorage();

	void checkAgingStructures();
	void checkNewlyBuiltStructures();

	// SAVE GAME MANAGEMENT FUNCTIONS
	void readRobots(NAS2D::Xml::XmlElement* element);
	void readStructures(NAS2D::Xml::XmlElement* element);
	void readTurns(NAS2D::Xml::XmlElement* element);
	void readPopulation(NAS2D::Xml::XmlElement* element);
	void readMoraleChanges(NAS2D::Xml::XmlElement*);

	void scrubRobotList();

	void load(const std::string& filePath);
	void save(const std::string& filePath);
	NAS2D::Xml::XmlElement* serializeProperties();

	// UI MANAGEMENT FUNCTIONS
	void clearMode();
	void clearSelections();

	void hideUi();
	void unhideUi();
	void initUi();
	void resetUi();

	bool modalUiElementDisplayed() const;

	void setupUiPositions(NAS2D::Vector<int> size);

	void populateRobotMenu();
	void populateStructureMenu();

	void updateStructuresAvailability();

	// UI EVENT HANDLERS
	void onTurns();
	void setOverlay(std::vector<Tile*>& tileList, Tile::Overlay overlay);
	void clearOverlays();
	void clearOverlay(std::vector<Tile*>& tileList);
	void updateOverlays();
	void changePoliceOverlayDepth(int oldDepth, int newDepth);
	void onToggleHeightmap();
	void onToggleConnectedness();
	void onToggleCommRangeOverlay();
	void onToggleRouteOverlay();
	void onTogglePoliceOverlay();

	void onNotificationClicked(const NotificationArea::Notification&);

	void onSaveGame();
	void onLoadGame();
	void onReturnToGame();
	void onGameOver();

	void onStructuresSelectionChange(const IconGrid::Item*);
	void onConnectionsSelectionChange(const IconGrid::Item*);
	void onRobotsSelectionChange(const IconGrid::Item*);

	void onDiggerSelectionDialog(Direction direction, Tile& tile);

	void onFileIoAction(const std::string& filePath, FileIo::FileOperation fileOp);

	void onTakeMeThere(const MapCoordinate& position);

private:
	TileMap* mTileMap{nullptr};
	CrimeRateUpdate mCrimeRateUpdate;
	CrimeExecution mCrimeExecution;

	StructureTracker mStructureTracker;

	ResearchTracker mResearchTracker;
	TechnologyCatalog mTechnologyReader;

	Planet::Attributes mPlanetAttributes;
	Difficulty mDifficulty = Difficulty::Medium;

	int mFood{0};

	// MISCELLANEOUS
	int mTurnCount = 0;

	int mCurrentMorale = constants::DefaultStartingMorale;
	int mPreviousMorale = constants::DefaultStartingMorale;

	int mLandersColonist = 0;
	int mLandersCargo = 0;

	int mResidentialCapacity = 0;

	// POOLS
	StorableResources mResourcesCount;
	RobotPool mRobotPool; /**< Robots that are currently available for use. */
	PopulationPool mPopulationPool;

	RobotTileTable mRobotList; /**< List of active robots and their positions on the map. */
	Population mPopulation;

	// ROUTING
	micropather::MicroPather* mPathSolver = nullptr;

	bool mLoadingExisting = false;
	std::string mExistingToLoad; /**< Filename of the existing game to load. */

	MainReportsUiState& mMainReportsState;
	std::unique_ptr<MapView> mMapView;

	const NAS2D::Image mUiIcons{"ui/icons.png"}; /**< User interface icons. */
	const NAS2D::Image mBackground{"sys/bg1.png"}; /**< Background image drawn behind the tile map. */

	MapCoordinate mMouseTilePosition;

	NAS2D::Rectangle<int> mMiniMapRect; /**< Area of the site map display. */

	InsertMode mInsertMode = InsertMode::None; /**< What's being inserted into the TileMap if anything. */
	StructureID mCurrentStructure = StructureID::SID_NONE; /**< Structure being placed. */
	Robot::Type mCurrentRobot = Robot::Type::None; /**< Robot being placed. */

	// USER INTERFACE
	Button mBtnTurns;
	Button mBtnToggleHeightmap;
	Button mBtnToggleConnectedness;
	Button mBtnToggleCommRangeOverlay;
	Button mBtnToggleRouteOverlay;
	Button mBtnTogglePoliceOverlay;

	// Bare Control's use for ToolTips
	Control mTooltipSystemButton;
	Control mTooltipCurrentTurns;

	ToolTip mToolTip;

	IconGrid mStructures{"ui/structures.png", 46, constants::MarginTight};
	IconGrid mRobots{"ui/robots.png", 46, constants::MarginTight};
	IconGrid mConnections{"ui/structures.png", 46, constants::MarginTight};

	CheatMenu mCheatMenu;
	DiggerDirection mDiggerDirection;
	FactoryProduction mFactoryProduction;
	FileIo mFileIoDialog;
	GameOverDialog mGameOverDialog;
	GameOptionsDialog mGameOptionsDialog;
	MajorEventAnnouncement mAnnouncement;
	MineOperationsWindow mMineOperationsWindow;
	NotificationArea mNotificationArea;
	NotificationWindow mNotificationWindow;
	PopulationPanel mPopulationPanel;
	ResourceBreakdownPanel mResourceBreakdownPanel;
	RobotInspector mRobotInspector;
	StructureInspector mStructureInspector;
	TileInspector mTileInspector;
	WarehouseInspector mWarehouseInspector;

	WindowStack mWindowStack;

	NAS2D::Rectangle<int> mBottomUiRect;

	// SIGNALS
	QuitSignal mQuitSignal;
	ReportsUiSignal mReportsUiSignal;
	MapChangedSignal mMapChangedSignal;

	std::vector<Tile*> mConnectednessOverlay;
	std::vector<Tile*> mCommRangeOverlay;
	std::vector<std::vector<Tile*>> mPoliceOverlays;
	std::vector<Tile*> mTruckRouteOverlay;

	ResourceInfoBar mResourceInfoBar;
	RobotDeploymentSummary mRobotDeploymentSummary;
	std::unique_ptr<MiniMap> mMiniMap;
	std::unique_ptr<DetailMap> mDetailMap;
	std::unique_ptr<NavControl> mNavControl;

	NAS2D::Fade mFade;
};
