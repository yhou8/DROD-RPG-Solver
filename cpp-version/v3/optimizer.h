#include <boost/preprocessor.hpp>

#define PlayerStructID						0
#define PlayerProgressDiffStructID			1
#define PlayerProgressStructID				2
#define PlayerObjectiveStructID				3
#define PlayerScoreStructID					4
#define PlayerStatStructID					5
#define PlayerCombatStructID				6
#define LevelStatStructID					7
#define MonsterStatStructID					8
#define EquipStatStructID					9

// --------------------------------------------------------------------------------

#define IGNORE								1

// --------------------------------------------------------------------------------

#define BOOST_PP_EXPR_IF_NOT(COND, EXPR)						BOOST_PP_IF(COND, , EXPR)

// --------------------------------------------------------------------------------

#define STRUCT_ID(STRUCT_NAME)									STRUCT_NAME ## StructID
#define STRUCT_EQUAL_R(R, S, T)									BOOST_PP_EQUAL(STRUCT_ID(S), STRUCT_ID(T))
#define STRUCT_SEQ_CONTAINS(SEQ, DATA)							BOOST_PP_SEQ_SIZE(BOOST_PP_SEQ_FILTER(STRUCT_EQUAL_R, DATA, SEQ))

#define MEMBER_ID(MEMBER_NAME)									MEMBER_NAME ## MemberID
#define MEMBER_EQUAL(S, T)										BOOST_PP_EQUAL(MEMBER_ID(S), MEMBER_ID(T))

#define M_STRUCT(ELEM)											BOOST_PP_SEQ_ELEM(0, ELEM)
#define M_TYPE(ELEM)											BOOST_PP_SEQ_ELEM(1, ELEM)
#define M_IDENTIFIER(ELEM)										BOOST_PP_SEQ_ELEM(2, ELEM)
#define M_DEFAULT(ELEM)											BOOST_PP_SEQ_ELEM(3, ELEM)
#define M_IGNORE(ELEM)											BOOST_PP_SEQ_ELEM(4, ELEM)

// --------------------------------------------------------------------------------

#define FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, LIST)				BOOST_PP_SEQ_FOR_EACH(FILTER_MEMBER_BY_STRUCT_HELPER, STRUCT_NAME, LIST)
#define FILTER_MEMBER_BY_STRUCT_HELPER(R, DATA, ELEM)			BOOST_PP_EXPR_IF(STRUCT_SEQ_CONTAINS(M_STRUCT(ELEM), DATA), (ELEM))

#define FILTER_MEMBER_BY_TWO_STRUCT(STRUCT_A, STRUCT_B, LIST)	BOOST_PP_SEQ_FOR_EACH(FILTER_MEMBER_BY_TWO_STRUCT_HELPER, (STRUCT_A) (STRUCT_B), LIST)
#define FILTER_MEMBER_BY_TWO_STRUCT_HELPER(R, DATA, ELEM)		BOOST_PP_EXPR_IF(BOOST_PP_AND( \
																	STRUCT_SEQ_CONTAINS(M_STRUCT(ELEM), BOOST_PP_SEQ_ELEM(0, DATA)), \
																	STRUCT_SEQ_CONTAINS(M_STRUCT(ELEM), BOOST_PP_SEQ_ELEM(1, DATA)) \
																), (ELEM))

#define FILTER_MEMBER_BY_MEMBER(MEMBER_NAME, LIST)				BOOST_PP_SEQ_FOR_EACH(FILTER_MEMBER_BY_MEMBER_HELPER, MEMBER_NAME, LIST)
#define FILTER_MEMBER_BY_MEMBER_HELPER(R, DATA, ELEM)			BOOST_PP_EXPR_IF(MEMBER_EQUAL(DATA, M_IDENTIFIER(ELEM)), (ELEM))

#define FILTER_MEMBER_BY_IGNORE(LIST)							BOOST_PP_SEQ_FOR_EACH(FILTER_MEMBER_BY_IGNORE_HELPER, MEMBER_NAME, LIST)
#define FILTER_MEMBER_BY_IGNORE_HELPER(R, DATA, ELEM)			BOOST_PP_EXPR_IF_NOT(M_IGNORE(ELEM), (ELEM))

#define MEMBER_IS_IGNORED(MEMBER_NAME, LIST)					BOOST_PP_NOT(BOOST_PP_SEQ_SIZE(FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_MEMBER(MEMBER_NAME, LIST))))

// --------------------------------------------------------------------------------

#define IDENTITY_INITIALIZATION(ELEM)							M_IDENTIFIER(ELEM): M_IDENTIFIER(ELEM),
#define EMPTY_INITIALIZATION(ELEM)								

#define STRUCT_MEMBER_DECLARATION_HELPER(R, DATA, ELEM)			BOOST_PP_EXPR_IF(M_IGNORE(ELEM), constexpr static) M_TYPE(ELEM) M_IDENTIFIER(ELEM) = M_DEFAULT(ELEM);
#define STRUCT_MEMBER_DECLARATION(STRUCT_NAME)					BOOST_PP_SEQ_FOR_EACH(STRUCT_MEMBER_DECLARATION_HELPER, _, FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, MEMBER_LIST))

#define HASH_FUNCTION_HELPER(STRUCT_NAME)						STRUCT_NAME ## Hash
#define HASH_FUNCTION(STRUCT_NAME) 								template<> struct std::hash<STRUCT_NAME> { inline std::size_t operator()(STRUCT_NAME const& t) const noexcept { return HASH_FUNCTION_HELPER(STRUCT_NAME); } };

#define CONVERT_STRUCT_DECLARATION(STRUCT_TO)					inline explicit operator STRUCT_TO() const;
#define CONVERT_STRUCT_HELPER(R, DATA, ELEM)					BOOST_PP_IF(M_IGNORE(ELEM), EMPTY_INITIALIZATION, IDENTITY_INITIALIZATION) (ELEM)
#define CONVERT_STRUCT(STRUCT_FROM, STRUCT_TO)					STRUCT_FROM::operator STRUCT_TO() const { return STRUCT_TO{ BOOST_PP_SEQ_FOR_EACH(CONVERT_STRUCT_HELPER, _, FILTER_MEMBER_BY_TWO_STRUCT(STRUCT_FROM, STRUCT_TO, MEMBER_LIST)) }; }

#define MEMBER_CODE_HELPER(R, DATA, ELEM)						DATA
#define MEMBER_CODE(STRUCT_NAME, MEMBER_NAME, VALUE)			BOOST_PP_SEQ_FOR_EACH(MEMBER_CODE_HELPER, VALUE, FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, FILTER_MEMBER_BY_MEMBER(MEMBER_NAME, MEMBER_LIST))))

#define MEMBER_INIT_HELPER(R, DATA, ELEM)						M_IDENTIFIER(ELEM): (M_TYPE(ELEM)) DATA,
#define MEMBER_INIT(STRUCT_NAME, MEMBER_NAME, VALUE)			BOOST_PP_SEQ_FOR_EACH(MEMBER_INIT_HELPER, (VALUE), FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, FILTER_MEMBER_BY_MEMBER(MEMBER_NAME, MEMBER_LIST))))

#define MEMBER_INIT_MAX_HELPER(R, DATA, ELEM)					M_IDENTIFIER(ELEM): std::numeric_limits<M_TYPE(ELEM)>::max(),
#define MEMBER_INIT_MAX(STRUCT_NAME, MEMBER_NAME)				BOOST_PP_SEQ_FOR_EACH(MEMBER_INIT_MAX_HELPER, _, FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, FILTER_MEMBER_BY_MEMBER(MEMBER_NAME, MEMBER_LIST))))

#define MEMBER_INIT_CHECK_HELPER(R, DATA, ELEM)					M_IDENTIFIER(ELEM): (M_TYPE(ELEM)) (assert(DATA <= std::numeric_limits<M_TYPE(ELEM)>::max()), DATA),
#define MEMBER_INIT_CHECK(STRUCT_NAME, MEMBER_NAME, VALUE)		BOOST_PP_SEQ_FOR_EACH(MEMBER_INIT_CHECK_HELPER, (VALUE), FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, FILTER_MEMBER_BY_MEMBER(MEMBER_NAME, MEMBER_LIST))))


#define STRUCT_UNARY_OPERATOR_CODE(STRUCT_NAME, OPERATOR, DELIMITER)				BOOST_PP_SEQ_FOR_EACH(STRUCT_UNARY_OPERATOR_CODE_HELPER, (OPERATOR) (DELIMITER), FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, MEMBER_LIST)))
#define STRUCT_UNARY_OPERATOR_CODE_HELPER(R, DATA, ELEM)							IntFlagOperator::BOOST_PP_SEQ_ELEM(0, DATA)(M_IDENTIFIER(ELEM))BOOST_PP_SEQ_ELEM(1, DATA)

#define STRUCT_BINARY_OPERATOR_CODE(STRUCT_NAME, OPERATOR, DELIMITER)				BOOST_PP_SEQ_FOR_EACH(STRUCT_BINARY_OPERATOR_CODE_HELPER, (OPERATOR) (DELIMITER), FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, MEMBER_LIST)))
#define STRUCT_BINARY_OPERATOR_CODE_HELPER(R, DATA, ELEM)							IntFlagOperator::BOOST_PP_SEQ_ELEM(0, DATA)(M_IDENTIFIER(ELEM), p.M_IDENTIFIER(ELEM))BOOST_PP_SEQ_ELEM(1, DATA)

#define STRUCT_UNARY_OPERATOR_INIT_CODE(STRUCT_NAME, OPERATOR)						BOOST_PP_SEQ_FOR_EACH(STRUCT_UNARY_OPERATOR_INIT_CODE_HELPER, OPERATOR, FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, MEMBER_LIST)))
#define STRUCT_UNARY_OPERATOR_INIT_CODE_HELPER(R, DATA, ELEM)						M_IDENTIFIER(ELEM): IntFlagOperator::DATA(M_IDENTIFIER(ELEM)),

#define STRUCT_BINARY_OPERATOR_INIT_CODE(STRUCT_NAME, OPERATOR)						BOOST_PP_SEQ_FOR_EACH(STRUCT_BINARY_OPERATOR_INIT_CODE_HELPER, OPERATOR, FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(STRUCT_NAME, MEMBER_LIST)))
#define STRUCT_BINARY_OPERATOR_INIT_CODE_HELPER(R, DATA, ELEM)						M_IDENTIFIER(ELEM): IntFlagOperator::DATA(M_IDENTIFIER(ELEM), p.M_IDENTIFIER(ELEM)),

#define STRUCT_ACTION_CODE(STRUCT_A, STRUCT_B, OPERATOR, DELIMITER)					BOOST_PP_SEQ_FOR_EACH(STRUCT_ACTION_CODE_HELPER, (OPERATOR) (DELIMITER), FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_TWO_STRUCT(STRUCT_A, STRUCT_B, MEMBER_LIST)))
#define STRUCT_ACTION_CODE_HELPER(R, DATA, ELEM)									IntFlagOperator::BOOST_PP_SEQ_ELEM(0, DATA)(M_IDENTIFIER(ELEM), p.M_IDENTIFIER(ELEM))BOOST_PP_SEQ_ELEM(1, DATA)

#define STRUCT_ACTION_INIT_CODE(STRUCT_A, STRUCT_B, OPERATOR)						BOOST_PP_SEQ_FOR_EACH(STRUCT_BINARY_OPERATOR_INIT_CODE_HELPER, OPERATOR, FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_TWO_STRUCT(STRUCT_A, STRUCT_B, MEMBER_LIST)))
#define STRUCT_ACTION_INIT_CODE_HELPER(R, DATA, ELEM)								M_IDENTIFIER(ELEM): IntFlagOperator::DATA(M_IDENTIFIER(ELEM), p.M_IDENTIFIER(ELEM)),

// --------------------------------------------------------------------------------

#include <cstdint>
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;

#define ASSERT_WITH_MSG(cond, msg) do \
{ if (!(cond)) { std::ostringstream str; str << msg; std::cerr << str.str(); std::abort(); } \
} while(0)

#include <cassert>
#include <limits>
#include <type_traits>
#include <string>
#include <vector>
#include <list>
#include <queue>
#include <map>
#include <unordered_map>
#include <iostream>
#include <sstream>
#include <fstream>

struct Player;
struct PlayerScore;
struct PlayerProgress;
struct PlayerProgressDiff;
struct PlayerObjective;
struct PlayerStat;
struct PlayerCombat;
struct Level;
struct Room;

// --------------------------------------------------------------------------------

#include "bitset.h"
#include "timer.h"

#define XSTR(x) #x
#define STR(x) XSTR(x)

#ifdef CustomObjective
	#include STR(../CustomObjective)
#else
	#include "objective.cpp"
#endif
#ifdef CustomStat
	#include STR(../CustomStat)
#else
	#include "stat.cpp"
#endif
#ifdef CustomMonster
	#include STR(../CustomMonster)
#else
	#include "monster.cpp"
#endif
#ifdef CustomHPBoost
	#include STR(../CustomHPBboost)
#else
	#include "hpboost.cpp"
#endif
#ifdef CustomEquipment
	#include STR(../CustomEquipment)
#else
	#include "equipment.cpp"
#endif
#ifdef CustomRoom
	#include STR(../CustomRoom)
#else
	#include "room.cpp"
#endif
#ifdef CustomLevel
	#include STR(../CustomLevel)
#else
	#include "level.h"
#endif
#ifdef CustomPlayer
	#include STR(../CustomPlayer)
#else
	#include "player.cpp"
#endif
#ifdef CustomSearch
	#include STR(../CustomSearch)
#else
	#include "search.cpp"
#endif
