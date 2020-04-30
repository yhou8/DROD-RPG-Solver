constexpr RoomElement newMonster(const char* name, int HP, int ATK, int DEF, int GR, unsigned int flag = 0) {
	auto s = Stat{
		.flag = flag,
		.HP = HP,
		.ATK = ATK,
		.DEF = DEF,
		.GR = GR,
		.REP = GR // Use GR for REP for now
	};
	return RoomElement{type: monster, stat: s};
}

constexpr RoomElement newMonsterHitFirst(const char* name, int HP, int ATK, int DEF, int GR, unsigned int flag = 0) {
	auto s = Stat{
		.flag = flag,
		.HP = HP,
		.ATK = ATK,
		.DEF = DEF,
		.GR = GR,
		.REP = GR // Use GR for REP for now
	};
	return RoomElement{type: monsterHitFirst, stat: s};
}

constexpr RoomElement newMonsterHitLate(const char* name, int HP, int ATK, int DEF, int GR, unsigned int flag = 0) {
	auto s = Stat{
		.flag = flag,
		.HP = HP,
		.ATK = ATK,
		.DEF = DEF,
		.GR = GR,
		.REP = GR // Use GR for REP for now
	};
	return RoomElement{type: monsterHitLate, stat: s};
}

constexpr auto DBrain = newMonster("Brain", 35, 9, 1, 1);
constexpr auto DRoach = newMonster("Roach", 45, 20, 2, 2);
constexpr auto DWraithwing = newMonster("Wraithwing", 35, 38, 3, 3);
constexpr auto DEvilEye = newMonster("Evil eye", 60, 32, 8, 5);
constexpr auto DRoachQueen = newMonster("Roach queen", 50, 42, 6, 6);
constexpr auto DSpider = newMonster("Spider", 55, 52, 12, 8);
constexpr auto DMudBaby = newMonster("Mud baby", 130, 60, 3, 8);
constexpr auto DAntlion = newMonster("Antlion", 60, 100, 8, 12);
constexpr auto DMudMother = newMonster("Mud mother", 50, 48, 22, 12);
constexpr auto DTarMother = newMonster("Tar mother", 100, 180, 110, 100);
constexpr auto DGelMother = newMonster("Gel mother", 180, 460, 360, 200);
constexpr auto DGrayMan = newMonster("Gray man", 260, 85, 5, 18);
constexpr auto DMadEye = newMonster("Mad eye", 100, 95, 30, 22);
constexpr auto DNeather = newMonster("Neather", 100, 65, 15, 25);
constexpr auto DRockGolem = newMonster("Rock golem", 20, 100, 68, 28);
constexpr auto DGoblin = newMonster("Goblin", 320, 120, 15, 30, DIsGoblin);
constexpr auto DTarBaby = newMonster("Tar baby", 320, 140, 20, 30);
constexpr auto DSoulless = newMonster("Soulless", 220, 180, 30, 35);
constexpr auto DMimic = newMonster("Mimic", 210, 200, 65, 45);
constexpr auto DSwordsman = newMonster("Swordsman", 100, 680, 50, 55);
constexpr auto DRedGuard = newMonster("Red guard", 160, 230, 105, 65);
constexpr auto DGelBaby = newMonster("Gel baby", 360, 310, 20, 40);
constexpr auto DFegundo = newMonster("Fegundo", 200, 390, 90, 50);
constexpr auto DWaterSkipper = newMonster("Water skipper", 220, 370, 110, 80);
constexpr auto DSeep = newMonster("Seep", 200, 380, 130, 90);
constexpr auto DPirate = newMonster("Pirate", 180, 430, 210, 120);
constexpr auto DAumtlich = newMonster("Aumtlich", 230, 450, 100, 100);
constexpr auto DWubba = newMonster("Wubba", 10, 0, 320, 100);
constexpr auto DGoblinKing = newMonster("Goblin king", 400, 199, 66, 144, DIsGoblin);
constexpr auto DSlayer = newMonster("Slayer", 4500, 560, 310, 1000);
constexpr auto DRockGiant = newMonster("Rock giant", 800, 500, 100, 500);
constexpr auto DRattlesnake = newMonster("Rattlesnake", 1200, 180, 20, 100);
constexpr auto DAdder = newMonster("Adder", 1500, 600, 250, 800);
constexpr auto DSerpent = newMonster("Serpent", 2500, 550, 350, 900);


constexpr auto DBrainHit = newMonsterHitFirst("Brain hit", 1, 9, 0, 0);
constexpr auto DRoachHit = newMonsterHitFirst("Roach hit", 1, 20, 0, 0);
constexpr auto DWraithwingHit = newMonsterHitFirst("Wraithwing hit", 1, 38, 0, 0);
constexpr auto DEvilEyeHit = newMonsterHitFirst("Evil eye hit", 1, 32, 0, 0);
constexpr auto DRoachQueenHit = newMonsterHitFirst("Roach queen hit", 1, 42, 0, 0);
constexpr auto DSpiderHit = newMonsterHitFirst("Spider hit", 1, 52, 0, 0);
constexpr auto DMudBabyHit = newMonsterHitFirst("Mud baby hit", 1, 60, 0, 0);
constexpr auto DAntlionHit = newMonsterHitFirst("Antlion hit", 1, 100, 0, 0);
constexpr auto DMudMotherHit = newMonsterHitFirst("Mud mother hit", 1, 48, 0, 0);
constexpr auto DTarMotherHit = newMonsterHitFirst("Tar mother hit", 1, 180, 0, 0);
constexpr auto DGelMotherHit = newMonsterHitFirst("Gel mother hit", 1, 460, 0, 0);
constexpr auto DGrayManHit = newMonsterHitFirst("Gray man hit", 1, 85, 0, 0);
constexpr auto DMadEyeHit = newMonsterHitFirst("Mad eye hit", 1, 95, 0, 0);
constexpr auto DNeatherHit = newMonsterHitFirst("Neather hit", 1, 65, 0, 0);
constexpr auto DRockGolemHit = newMonsterHitFirst("Rock golem hit", 1, 100, 0, 0);
constexpr auto DGoblinHit = newMonsterHitFirst("Goblin hit", 1, 120, 0, 0, DIsGoblin);
constexpr auto DTarBabyHit = newMonsterHitFirst("Tar baby hit", 1, 140, 0, 0);
constexpr auto DSoullessHit = newMonsterHitFirst("Soulless hit", 1, 180, 0, 0);
constexpr auto DMimicHit = newMonsterHitFirst("Mimic hit", 1, 200, 0, 0);
constexpr auto DSwordsmanHit = newMonsterHitFirst("Swordsman hit", 1, 680, 0, 0);
constexpr auto DRedGuardHit = newMonsterHitFirst("Red guard hit", 1, 230, 0, 0);
constexpr auto DGelBabyHit = newMonsterHitFirst("Gel baby hit", 1, 310, 0, 0);
constexpr auto DFegundoHit = newMonsterHitFirst("Fegundo hit", 1, 390, 0, 0);
constexpr auto DWaterSkipperHit = newMonsterHitFirst("Water skipper hit", 1, 370, 0, 0);
constexpr auto DSeepHit = newMonsterHitFirst("Seep hit", 1, 380, 0, 0);
constexpr auto DPirateHit = newMonsterHitFirst("Pirate hit", 1, 430, 0, 0);
constexpr auto DAumtlichHit = newMonsterHitFirst("Aumtlich hit", 1, 450, 0, 0);
constexpr auto DWubbaHit = newMonsterHitFirst("Wubba hit", 1, 0, 0, 0);
constexpr auto DGoblinKingHit = newMonsterHitFirst("Goblin king hit", 1, 199, 0, 0, DIsGoblin);
constexpr auto DSlayerHit = newMonsterHitFirst("Slayer hit", 1, 560, 0, 0);
constexpr auto DRockGiantHit = newMonsterHitFirst("Rock giant hit", 1, 500, 0, 0);
constexpr auto DRattlesnakeHit = newMonsterHitFirst("Rattlesnake hit", 1, 180, 0, 0);
constexpr auto DAdderHit = newMonsterHitFirst("Adder hit", 1, 600, 0, 0);
constexpr auto DSerpentHit = newMonsterHitFirst("Serpent hit", 1, 550, 0, 0);


constexpr auto DBrainHitFirst = newMonsterHitFirst("Brain (attack first)", 35, 9, 1, 1);
constexpr auto DRoachHitFirst = newMonsterHitFirst("Roach (attack first)", 45, 20, 2, 2);
constexpr auto DWraithwingHitFirst = newMonsterHitFirst("Wraithwing (attack first)", 35, 38, 3, 3);
constexpr auto DEvilEyeHitFirst = newMonsterHitFirst("Evil eye (attack first)", 60, 32, 8, 5);
constexpr auto DRoachQueenHitFirst = newMonsterHitFirst("Roach queen (attack first)", 50, 42, 6, 6);
constexpr auto DSpiderHitFirst = newMonsterHitFirst("Spider (attack first)", 55, 52, 12, 8);
constexpr auto DMudBabyHitFirst = newMonsterHitFirst("Mud baby (attack first)", 130, 60, 3, 8);
constexpr auto DAntlionHitFirst = newMonsterHitFirst("Antlion (attack first)", 60, 100, 8, 12);
constexpr auto DMudMotherHitFirst = newMonsterHitFirst("Mud mother (attack first)", 50, 48, 22, 12);
constexpr auto DTarMotherHitFirst = newMonsterHitFirst("Tar mother (attack first)", 100, 180, 110, 100);
constexpr auto DGelMotherHitFirst = newMonsterHitFirst("Gel mother (attack first)", 180, 460, 360, 200);
constexpr auto DGrayManHitFirst = newMonsterHitFirst("Gray man (attack first)", 260, 85, 5, 18);
constexpr auto DMadEyeHitFirst = newMonsterHitFirst("Mad eye (attack first)", 100, 95, 30, 22);
constexpr auto DNeatherHitFirst = newMonsterHitFirst("Neather (attack first)", 100, 65, 15, 25);
constexpr auto DRockGolemHitFirst = newMonsterHitFirst("Rock golem (attack first)", 20, 100, 68, 28);
constexpr auto DGoblinHitFirst = newMonsterHitFirst("Goblin (attack first)", 320, 120, 15, 30, DIsGoblin);
constexpr auto DTarBabyHitFirst = newMonsterHitFirst("Tar baby (attack first)", 320, 140, 20, 30);
constexpr auto DSoullessHitFirst = newMonsterHitFirst("Soulless (attack first)", 220, 180, 30, 35);
constexpr auto DMimicHitFirst = newMonsterHitFirst("Mimic (attack first)", 210, 200, 65, 45);
constexpr auto DSwordsmanHitFirst = newMonsterHitFirst("Swordsman (attack first)", 100, 680, 50, 55);
constexpr auto DRedGuardHitFirst = newMonsterHitFirst("Red guard (attack first)", 160, 230, 105, 65);
constexpr auto DGelBabyHitFirst = newMonsterHitFirst("Gel baby (attack first)", 360, 310, 20, 40);
constexpr auto DFegundoHitFirst = newMonsterHitFirst("Fegundo (attack first)", 200, 390, 90, 50);
constexpr auto DWaterSkipperHitFirst = newMonsterHitFirst("Water skipper (attack first)", 220, 370, 110, 80);
constexpr auto DSeepHitFirst = newMonsterHitFirst("Seep (attack first)", 200, 380, 130, 90);
constexpr auto DPirateHitFirst = newMonsterHitFirst("Pirate (attack first)", 180, 430, 210, 120);
constexpr auto DAumtlichHitFirst = newMonsterHitFirst("Aumtlich (attack first)", 230, 450, 100, 100);
constexpr auto DWubbaHitFirst = newMonsterHitFirst("Wubba (attack first)", 10, 0, 320, 100);
constexpr auto DGoblinKingHitFirst = newMonsterHitFirst("Goblin king (attack first)", 400, 199, 66, 144, DIsGoblin);
constexpr auto DSlayerHitFirst = newMonsterHitFirst("Slayer (attack first)", 4500, 560, 310, 1000);
constexpr auto DRockGiantHitFirst = newMonsterHitFirst("Rock giant (attack first)", 800, 500, 100, 500);
constexpr auto DRattlesnakeHitFirst = newMonsterHitFirst("Rattlesnake (attack first)", 1200, 180, 20, 100);
constexpr auto DAdderHitFirst = newMonsterHitFirst("Adder (attack first)", 1500, 600, 250, 800);
constexpr auto DSerpentHitFirst = newMonsterHitFirst("Serpent (attack first)", 2500, 550, 350, 900);

constexpr auto DBrainedBrain = newMonster("Brained Brain", DBrain.stat.HP, DBrain.stat.ATK * 2, DBrain.stat.DEF, DBrain.stat.GR);
constexpr auto DBrainedRoach = newMonster("Brained Roach", DRoach.stat.HP, DRoach.stat.ATK * 2, DRoach.stat.DEF, DRoach.stat.GR);
constexpr auto DBrainedWraithwing = newMonster("Brained Wraithwing", DWraithwing.stat.HP, DWraithwing.stat.ATK * 2, DWraithwing.stat.DEF, DWraithwing.stat.GR);
constexpr auto DBrainedEvilEye = newMonster("Brained Evil eye", DEvilEye.stat.HP, DEvilEye.stat.ATK * 2, DEvilEye.stat.DEF, DEvilEye.stat.GR);
constexpr auto DBrainedRoachQueen = newMonster("Brained Roach queen", DRoachQueen.stat.HP, DRoachQueen.stat.ATK * 2, DRoachQueen.stat.DEF, DRoachQueen.stat.GR);
constexpr auto DBrainedSpider = newMonster("Brained Spider", DSpider.stat.HP, DSpider.stat.ATK * 2, DSpider.stat.DEF, DSpider.stat.GR);
constexpr auto DBrainedMudBaby = newMonster("Brained Mud baby", DMudBaby.stat.HP, DMudBaby.stat.ATK * 2, DMudBaby.stat.DEF, DMudBaby.stat.GR);
constexpr auto DBrainedAntlion = newMonster("Brained Antlion", DAntlion.stat.HP, DAntlion.stat.ATK * 2, DAntlion.stat.DEF, DAntlion.stat.GR);
constexpr auto DBrainedMudMother = newMonster("Brained Mud mother", DMudMother.stat.HP, DMudMother.stat.ATK * 2, DMudMother.stat.DEF, DMudMother.stat.GR);
constexpr auto DBrainedTarMother = newMonster("Brained Tar mother", DTarMother.stat.HP, DTarMother.stat.ATK * 2, DTarMother.stat.DEF, DTarMother.stat.GR);
constexpr auto DBrainedGelMother = newMonster("Brained Gel mother", DGelMother.stat.HP, DGelMother.stat.ATK * 2, DGelMother.stat.DEF, DGelMother.stat.GR);
constexpr auto DBrainedGrayMan = newMonster("Brained Gray man", DGrayMan.stat.HP, DGrayMan.stat.ATK * 2, DGrayMan.stat.DEF, DGrayMan.stat.GR);
constexpr auto DBrainedMadEye = newMonster("Brained Mad eye", DMadEye.stat.HP, DMadEye.stat.ATK * 2, DMadEye.stat.DEF, DMadEye.stat.GR);
constexpr auto DBrainedNeather = newMonster("Brained Neather", DNeather.stat.HP, DNeather.stat.ATK * 2, DNeather.stat.DEF, DNeather.stat.GR);
constexpr auto DBrainedRockGolem = newMonster("Brained Rock golem", DRockGolem.stat.HP, DRockGolem.stat.ATK * 2, DRockGolem.stat.DEF, DRockGolem.stat.GR);
constexpr auto DBrainedGoblin = newMonster("Brained Goblin", DGoblin.stat.HP, DGoblin.stat.ATK * 2, DGoblin.stat.DEF, DGoblin.stat.GR);
constexpr auto DBrainedTarBaby = newMonster("Brained Tar baby", DTarBaby.stat.HP, DTarBaby.stat.ATK * 2, DTarBaby.stat.DEF, DTarBaby.stat.GR);
constexpr auto DBrainedSoulless = newMonster("Brained Soulless", DSoulless.stat.HP, DSoulless.stat.ATK * 2, DSoulless.stat.DEF, DSoulless.stat.GR);
constexpr auto DBrainedMimic = newMonster("Brained Mimic", DMimic.stat.HP, DMimic.stat.ATK * 2, DMimic.stat.DEF, DMimic.stat.GR);
constexpr auto DBrainedSwordsman = newMonster("Brained Swordsman", DSwordsman.stat.HP, DSwordsman.stat.ATK * 2, DSwordsman.stat.DEF, DSwordsman.stat.GR);
constexpr auto DBrainedRedGuard = newMonster("Brained Red guard", DRedGuard.stat.HP, DRedGuard.stat.ATK * 2, DRedGuard.stat.DEF, DRedGuard.stat.GR);
constexpr auto DBrainedGelBaby = newMonster("Brained Gel baby", DGelBaby.stat.HP, DGelBaby.stat.ATK * 2, DGelBaby.stat.DEF, DGelBaby.stat.GR);
constexpr auto DBrainedFegundo = newMonster("Brained Fegundo", DFegundo.stat.HP, DFegundo.stat.ATK * 2, DFegundo.stat.DEF, DFegundo.stat.GR);
constexpr auto DBrainedWaterSkipper = newMonster("Brained Water skipper", DWaterSkipper.stat.HP, DWaterSkipper.stat.ATK * 2, DWaterSkipper.stat.DEF, DWaterSkipper.stat.GR);
constexpr auto DBrainedSeep = newMonster("Brained Seep", DSeep.stat.HP, DSeep.stat.ATK * 2, DSeep.stat.DEF, DSeep.stat.GR);
constexpr auto DBrainedPirate = newMonster("Brained Pirate", DPirate.stat.HP, DPirate.stat.ATK * 2, DPirate.stat.DEF, DPirate.stat.GR);
constexpr auto DBrainedAumtlich = newMonster("Brained Aumtlich", DAumtlich.stat.HP, DAumtlich.stat.ATK * 2, DAumtlich.stat.DEF, DAumtlich.stat.GR);
constexpr auto DBrainedWubba = newMonster("Brained Wubba", DWubba.stat.HP, DWubba.stat.ATK * 2, DWubba.stat.DEF, DWubba.stat.GR);
constexpr auto DBrainedGoblinKing = newMonster("Brained Goblin king", DGoblinKing.stat.HP, DGoblinKing.stat.ATK * 2, DGoblinKing.stat.DEF, DGoblinKing.stat.GR);
constexpr auto DBrainedSlayer = newMonster("Brained Slayer", DSlayer.stat.HP, DSlayer.stat.ATK * 2, DSlayer.stat.DEF, DSlayer.stat.GR);
constexpr auto DBrainedRockGiant = newMonster("Brained Rock giant", DRockGiant.stat.HP, DRockGiant.stat.ATK * 2, DRockGiant.stat.DEF, DRockGiant.stat.GR);
constexpr auto DBrainedRattlesnake = newMonster("Brained Rattlesnake", DRattlesnake.stat.HP, DRattlesnake.stat.ATK * 2, DRattlesnake.stat.DEF, DRattlesnake.stat.GR);
constexpr auto DBrainedAdder = newMonster("Brained Adder", DAdder.stat.HP, DAdder.stat.ATK * 2, DAdder.stat.DEF, DAdder.stat.GR);
constexpr auto DBrainedSerpent = newMonster("Brained Serpent", DSerpent.stat.HP, DSerpent.stat.ATK * 2, DSerpent.stat.DEF, DSerpent.stat.GR);


constexpr auto DBrainedBrainHitFirst = newMonsterHitFirst("Brained Brain", DBrain.stat.HP, DBrain.stat.ATK * 2, DBrain.stat.DEF, DBrain.stat.GR);
constexpr auto DBrainedRoachHitFirst = newMonsterHitFirst("Brained Roach", DRoach.stat.HP, DRoach.stat.ATK * 2, DRoach.stat.DEF, DRoach.stat.GR);
constexpr auto DBrainedWraithwingHitFirst = newMonsterHitFirst("Brained Wraithwing", DWraithwing.stat.HP, DWraithwing.stat.ATK * 2, DWraithwing.stat.DEF, DWraithwing.stat.GR);
constexpr auto DBrainedEvilEyeHitFirst = newMonsterHitFirst("Brained Evil eye", DEvilEye.stat.HP, DEvilEye.stat.ATK * 2, DEvilEye.stat.DEF, DEvilEye.stat.GR);
constexpr auto DBrainedRoachQueenHitFirst = newMonsterHitFirst("Brained Roach queen", DRoachQueen.stat.HP, DRoachQueen.stat.ATK * 2, DRoachQueen.stat.DEF, DRoachQueen.stat.GR);
constexpr auto DBrainedSpiderHitFirst = newMonsterHitFirst("Brained Spider", DSpider.stat.HP, DSpider.stat.ATK * 2, DSpider.stat.DEF, DSpider.stat.GR);
constexpr auto DBrainedMudBabyHitFirst = newMonsterHitFirst("Brained Mud baby", DMudBaby.stat.HP, DMudBaby.stat.ATK * 2, DMudBaby.stat.DEF, DMudBaby.stat.GR);
constexpr auto DBrainedAntlionHitFirst = newMonsterHitFirst("Brained Antlion", DAntlion.stat.HP, DAntlion.stat.ATK * 2, DAntlion.stat.DEF, DAntlion.stat.GR);
constexpr auto DBrainedMudMotherHitFirst = newMonsterHitFirst("Brained Mud mother", DMudMother.stat.HP, DMudMother.stat.ATK * 2, DMudMother.stat.DEF, DMudMother.stat.GR);
constexpr auto DBrainedTarMotherHitFirst = newMonsterHitFirst("Brained Tar mother", DTarMother.stat.HP, DTarMother.stat.ATK * 2, DTarMother.stat.DEF, DTarMother.stat.GR);
constexpr auto DBrainedGelMotherHitFirst = newMonsterHitFirst("Brained Gel mother", DGelMother.stat.HP, DGelMother.stat.ATK * 2, DGelMother.stat.DEF, DGelMother.stat.GR);
constexpr auto DBrainedGrayManHitFirst = newMonsterHitFirst("Brained Gray man", DGrayMan.stat.HP, DGrayMan.stat.ATK * 2, DGrayMan.stat.DEF, DGrayMan.stat.GR);
constexpr auto DBrainedMadEyeHitFirst = newMonsterHitFirst("Brained Mad eye", DMadEye.stat.HP, DMadEye.stat.ATK * 2, DMadEye.stat.DEF, DMadEye.stat.GR);
constexpr auto DBrainedNeatherHitFirst = newMonsterHitFirst("Brained Neather", DNeather.stat.HP, DNeather.stat.ATK * 2, DNeather.stat.DEF, DNeather.stat.GR);
constexpr auto DBrainedRockGolemHitFirst = newMonsterHitFirst("Brained Rock golem", DRockGolem.stat.HP, DRockGolem.stat.ATK * 2, DRockGolem.stat.DEF, DRockGolem.stat.GR);
constexpr auto DBrainedGoblinHitFirst = newMonsterHitFirst("Brained Goblin", DGoblin.stat.HP, DGoblin.stat.ATK * 2, DGoblin.stat.DEF, DGoblin.stat.GR);
constexpr auto DBrainedTarBabyHitFirst = newMonsterHitFirst("Brained Tar baby", DTarBaby.stat.HP, DTarBaby.stat.ATK * 2, DTarBaby.stat.DEF, DTarBaby.stat.GR);
constexpr auto DBrainedSoullessHitFirst = newMonsterHitFirst("Brained Soulless", DSoulless.stat.HP, DSoulless.stat.ATK * 2, DSoulless.stat.DEF, DSoulless.stat.GR);
constexpr auto DBrainedMimicHitFirst = newMonsterHitFirst("Brained Mimic", DMimic.stat.HP, DMimic.stat.ATK * 2, DMimic.stat.DEF, DMimic.stat.GR);
constexpr auto DBrainedSwordsmanHitFirst = newMonsterHitFirst("Brained Swordsman", DSwordsman.stat.HP, DSwordsman.stat.ATK * 2, DSwordsman.stat.DEF, DSwordsman.stat.GR);
constexpr auto DBrainedRedGuardHitFirst = newMonsterHitFirst("Brained Red guard", DRedGuard.stat.HP, DRedGuard.stat.ATK * 2, DRedGuard.stat.DEF, DRedGuard.stat.GR);
constexpr auto DBrainedGelBabyHitFirst = newMonsterHitFirst("Brained Gel baby", DGelBaby.stat.HP, DGelBaby.stat.ATK * 2, DGelBaby.stat.DEF, DGelBaby.stat.GR);
constexpr auto DBrainedFegundoHitFirst = newMonsterHitFirst("Brained Fegundo", DFegundo.stat.HP, DFegundo.stat.ATK * 2, DFegundo.stat.DEF, DFegundo.stat.GR);
constexpr auto DBrainedWaterSkipperHitFirst = newMonsterHitFirst("Brained Water skipper", DWaterSkipper.stat.HP, DWaterSkipper.stat.ATK * 2, DWaterSkipper.stat.DEF, DWaterSkipper.stat.GR);
constexpr auto DBrainedSeepHitFirst = newMonsterHitFirst("Brained Seep", DSeep.stat.HP, DSeep.stat.ATK * 2, DSeep.stat.DEF, DSeep.stat.GR);
constexpr auto DBrainedPirateHitFirst = newMonsterHitFirst("Brained Pirate", DPirate.stat.HP, DPirate.stat.ATK * 2, DPirate.stat.DEF, DPirate.stat.GR);
constexpr auto DBrainedAumtlichHitFirst = newMonsterHitFirst("Brained Aumtlich", DAumtlich.stat.HP, DAumtlich.stat.ATK * 2, DAumtlich.stat.DEF, DAumtlich.stat.GR);
constexpr auto DBrainedWubbaHitFirst = newMonsterHitFirst("Brained Wubba", DWubba.stat.HP, DWubba.stat.ATK * 2, DWubba.stat.DEF, DWubba.stat.GR);
constexpr auto DBrainedGoblinKingHitFirst = newMonsterHitFirst("Brained Goblin king", DGoblinKing.stat.HP, DGoblinKing.stat.ATK * 2, DGoblinKing.stat.DEF, DGoblinKing.stat.GR);
constexpr auto DBrainedSlayerHitFirst = newMonsterHitFirst("Brained Slayer", DSlayer.stat.HP, DSlayer.stat.ATK * 2, DSlayer.stat.DEF, DSlayer.stat.GR);
constexpr auto DBrainedRockGiantHitFirst = newMonsterHitFirst("Brained Rock giant", DRockGiant.stat.HP, DRockGiant.stat.ATK * 2, DRockGiant.stat.DEF, DRockGiant.stat.GR);
constexpr auto DBrainedRattlesnakeHitFirst = newMonsterHitFirst("Brained Rattlesnake", DRattlesnake.stat.HP, DRattlesnake.stat.ATK * 2, DRattlesnake.stat.DEF, DRattlesnake.stat.GR);
constexpr auto DBrainedAdderHitFirst = newMonsterHitFirst("Brained Adder", DAdder.stat.HP, DAdder.stat.ATK * 2, DAdder.stat.DEF, DAdder.stat.GR);
constexpr auto DBrainedSerpentHitFirst = newMonsterHitFirst("Brained Serpent", DSerpent.stat.HP, DSerpent.stat.ATK * 2, DSerpent.stat.DEF, DSerpent.stat.GR);


constexpr auto DRoachEgg = newMonster("Roach Egg", 1, 0, 23, 0);

constexpr auto DMadEyeBack = newMonsterHitLate("Mad eye (back)", DMadEye.stat.HP, DMadEye.stat.ATK, DMadEye.stat.DEF, DMadEye.stat.GR);
constexpr auto DEvilEyeBack = newMonsterHitLate("Evil eye (back)", DEvilEye.stat.HP, DEvilEye.stat.ATK, DEvilEye.stat.DEF, DEvilEye.stat.GR);
constexpr auto DBrainedMadEyeHit = newMonsterHitFirst("Mad eye hit", 1, DBrainedMadEye.stat.ATK, 0, 0);
constexpr auto DBrainedEvilEyeHit = newMonsterHitFirst("Evil eye hit", 1, DBrainedEvilEye.stat.ATK, 0, 0);
constexpr auto DBrainedBrainedBrain = newMonster("Brained Brain", DBrain.stat.HP, DBrain.stat.ATK * 4, DBrain.stat.DEF, DBrain.stat.GR);

RoomStat monsterToRoomStat(const Stat& monster, const Stat& player, bool hitFirst, bool hitLate) {
	int HPCost;
	int effATK = (player.flag & DDoubleATKAgainstGoblin) && (monster.flag & DIsGoblin) || (player.flag & DDoubleATKAgainstWyrm) && (monster.flag & DIsWyrm) ? player.ATK * 2 : player.ATK;
	Stat effMonster = monster;
	if ((player.flag & DWeakenZombie) && (monster.flag & DIsZombie)) {
		effMonster = {HP: (monster.HP + 1) / 2, ATK: monster.ATK, DEF: (monster.DEF + 1) / 2, GR: monster.GR, REP: monster.REP};
	}
	if (effATK <= effMonster.DEF) {
		HPCost = 1 << 20;
	} else if (player.DEF >= effMonster.ATK) {
		HPCost = 0;
	} else {
		int hits = (effMonster.HP - 1) / (effATK - effMonster.DEF);
		if (hitFirst || !(player.flag & DHasWeapon)) {
			++hits;
		}
		if (hitLate && hits) {
			--hits;
		}
		HPCost = hits * (effMonster.ATK - player.DEF);
	}
	Stat diff = {HP: -HPCost,
		GR: player.flag & (DDoubleGRWeapon | DDoubleGRAccessory) ? effMonster.GR * 2 : effMonster.GR,
		REP: player.flag & DDoubleREPAccessory ? effMonster.REP * 2 : effMonster.REP
	};
	Stat req = {HP: HPCost};
	return RoomStat{diff, req};
}
