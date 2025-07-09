#include <IbPinyin/pinyin.hpp>

#define LITERAL(s) IB_PINYIN_LITERAL(s)

namespace pinyin{
    String Pinyin::to_double_pinyin_abc() const
    {
#define ITEM(k, v) { LITERAL(#k), LITERAL(#v) }
        static std::unordered_map<StringView, StringView> pinyin_map{
            ITEM(e, oe), ITEM(o, oo),
            ITEM(a, oa),
            ITEM(ei, oq),
            ITEM(ai, ol),
            ITEM(ou, ob),
            ITEM(ao, ok),
            ITEM(en, of),
            ITEM(an, oj),
            ITEM(eng, og),
            ITEM(ang, oh)
        };
        static std::unordered_map<StringView, StringView> initial_map{
            ITEM(zh, a), ITEM(ch, e), ITEM(sh, v),
        };
        static std::unordered_map<StringView, StringView> final_map{
            ITEM(i, i), ITEM(u, u), ITEM(v, v),
            ITEM(e, e), ITEM(ie, x), ITEM(o, o), ITEM(uo, o), ITEM(ue, m), ITEM(ve, m),
            ITEM(a, a), ITEM(ia, d), ITEM(ua, d),
            ITEM(ei, q), ITEM(ui, m),
            ITEM(ai, l), ITEM(uai, c),
            ITEM(ou, b), ITEM(iu, r),
            ITEM(ao, k), ITEM(iao, z),
            ITEM(in, c), ITEM(un, n), ITEM(vn, n),
            ITEM(en, f),
            ITEM(an, j), ITEM(ian, w), ITEM(uan, p), ITEM(van, p),
            ITEM(ing, y),
            ITEM(ong, s), ITEM(iong, s),
            ITEM(eng, g),
            ITEM(ang, h), ITEM(iang, t), ITEM(uang, t),
            ITEM(er, or)
        };
#undef ITEM
        return convert(pinyin_map, initial_map, final_map);
    }

    String Pinyin::to_double_pinyin_jiajia() const
    {
#define ITEM(k, v) { LITERAL(#k), LITERAL(#v) }
        static std::unordered_map<StringView, StringView> pinyin_map{
            ITEM(e, ee), ITEM(o, oo),
            ITEM(a, aa),
            ITEM(ei, ew),
            ITEM(ai, as),
            ITEM(ou, op),
            ITEM(ao, ad),
            ITEM(en, er),
            ITEM(an, af),
            ITEM(eng, et),
            ITEM(ang, ag)
        };
        static std::unordered_map<StringView, StringView> initial_map{
            ITEM(zh, v), ITEM(ch, u), ITEM(sh, i),
        };
        static std::unordered_map<StringView, StringView> final_map{
            ITEM(i, i), ITEM(u, u), ITEM(v, v),
            ITEM(e, e), ITEM(ie, m), ITEM(o, o), ITEM(uo, o), ITEM(ue, x), ITEM(ve, t),
            ITEM(a, a), ITEM(ia, b), ITEM(ua, b),
            ITEM(ei, w), ITEM(ui, v),
            ITEM(ai, s), ITEM(uai, x),
            ITEM(ou, p), ITEM(iu, n),
            ITEM(ao, d), ITEM(iao, k),
            ITEM(in, l), ITEM(un, z), ITEM(vn, z),
            ITEM(en, r),
            ITEM(an, f), ITEM(ian, j), ITEM(uan, c), ITEM(van, c),
            ITEM(ing, q),
            ITEM(ong, y), ITEM(iong, y),
            ITEM(eng, t),
            ITEM(ang, g), ITEM(iang, h), ITEM(uang, h),
            ITEM(er, eq)
        };
#undef ITEM
        return convert(pinyin_map, initial_map, final_map);
    }

    String Pinyin::to_double_pinyin_microsoft() const
    {
#define ITEM(k, v) { LITERAL(#k), LITERAL(#v) }
        static std::unordered_map<StringView, StringView> pinyin_map{
            ITEM(e, oe), ITEM(o, oo),
            ITEM(a, oa),
            ITEM(ei, oz),
            ITEM(ai, ol),
            ITEM(ou, ob),
            ITEM(ao, ok),
            ITEM(en, of),
            ITEM(an, oj),
            ITEM(eng, og),
            ITEM(ang, oh)
        };
        static std::unordered_map<StringView, StringView> initial_map{
            ITEM(zh, v), ITEM(ch, i), ITEM(sh, u),
        };
        static std::unordered_map<StringView, StringView> final_map{
            ITEM(i, i), ITEM(u, u), ITEM(v, y),
            ITEM(e, e), ITEM(ie, x), ITEM(o, o), ITEM(uo, o), ITEM(ue, t), ITEM(ve, v),
            ITEM(a, a), ITEM(ia, w), ITEM(ua, w),
            ITEM(ei, z), ITEM(ui, v),
            ITEM(ai, l), ITEM(uai, y),
            ITEM(ou, b), ITEM(iu, q),
            ITEM(ao, k), ITEM(iao, c),
            ITEM(in, n), ITEM(un, p), ITEM(vn, p),
            ITEM(en, f),
            ITEM(an, j), ITEM(ian, m), ITEM(uan, r), ITEM(van, r),
            ITEM(ing, ;),
            ITEM(ong, s), ITEM(iong, s),
            ITEM(eng, g),
            ITEM(ang, h), ITEM(iang, d), ITEM(uang, d),
            ITEM(er, or)
        };
#undef ITEM
        return convert(pinyin_map, initial_map, final_map);
    }

    String Pinyin::to_double_pinyin_thunisoft() const
    {
#define ITEM(k, v) { LITERAL(#k), LITERAL(#v) }
        static std::unordered_map<StringView, StringView> pinyin_map{
            ITEM(e, oe), ITEM(o, oo),
            ITEM(a, oa),
            ITEM(ei, ok),
            ITEM(ai, op),
            ITEM(ou, oz),
            ITEM(ao, oq),
            ITEM(en, ow),
            ITEM(an, or),
            ITEM(eng, ot),
            ITEM(ang, os)
        };
        static std::unordered_map<StringView, StringView> initial_map{
            ITEM(zh, u), ITEM(ch, a), ITEM(sh, i),
        };
        static std::unordered_map<StringView, StringView> final_map{
            ITEM(i, i), ITEM(u, u), ITEM(v, v),
            ITEM(e, e), ITEM(ie, d), ITEM(o, o), ITEM(uo, o), ITEM(ue, n), ITEM(ve, n),
            ITEM(a, a), ITEM(ia, x), ITEM(ua, x),
            ITEM(ei, k), ITEM(ui, n),
            ITEM(ai, p), ITEM(uai, y),
            ITEM(ou, z), ITEM(iu, j),
            ITEM(ao, q), ITEM(iao, b),
            ITEM(in, y), ITEM(un, m), ITEM(vn, y),
            ITEM(en, w),
            ITEM(an, r), ITEM(ian, f), ITEM(uan, l), ITEM(van, l),
            ITEM(ing, ;),
            ITEM(ong, h), ITEM(iong, h),
            ITEM(eng, t),
            ITEM(ang, s), ITEM(iang, g), ITEM(uang, g),
            ITEM(er, oj)
        };
#undef ITEM
        return convert(pinyin_map, initial_map, final_map);
    }

    String Pinyin::to_double_pinyin_xiaohe() const
    {
#define ITEM(k, v) { LITERAL(#k), LITERAL(#v) }
        static std::unordered_map<StringView, StringView> pinyin_map{
            ITEM(e, ee), ITEM(o, oo),
            ITEM(a, aa),
            ITEM(ei, ei),
            ITEM(ai, ai),
            ITEM(ou, ou),
            ITEM(ao, ao),
            ITEM(en, en),
            ITEM(an, an),
            ITEM(eng, eg),
            ITEM(ang, ah)
        };
        static std::unordered_map<StringView, StringView> initial_map{
            ITEM(zh, v), ITEM(ch, i), ITEM(sh, u),
        };
        static std::unordered_map<StringView, StringView> final_map{
            ITEM(i, i), ITEM(u, u), ITEM(v, v),
            ITEM(e, e), ITEM(ie, p), ITEM(o, o), ITEM(uo, o), ITEM(ue, t), ITEM(ve, t),
            ITEM(a, a), ITEM(ia, x), ITEM(ua, x),
            ITEM(ei, w), ITEM(ui, v),
            ITEM(ai, d), ITEM(uai, k),
            ITEM(ou, z), ITEM(iu, q),
            ITEM(ao, c), ITEM(iao, n),
            ITEM(in, b), ITEM(un, y), ITEM(vn, y),
            ITEM(en, f),
            ITEM(an, j), ITEM(ian, m), ITEM(uan, r), ITEM(van, r),
            ITEM(ing, k),
            ITEM(ong, s), ITEM(iong, s),
            ITEM(eng, g),
            ITEM(ang, h), ITEM(iang, l), ITEM(uang, l),
            ITEM(er, er)
        };
#undef ITEM
        return convert(pinyin_map, initial_map, final_map);
    }

    String Pinyin::to_double_pinyin_zrm() const
    {
#define ITEM(k, v) { LITERAL(#k), LITERAL(#v) }
        static std::unordered_map<StringView, StringView> pinyin_map{
            ITEM(e, ee), ITEM(o, oo),
            ITEM(a, aa),
            ITEM(ei, ei),
            ITEM(ai, ai),
            ITEM(ou, ou),
            ITEM(ao, ao),
            ITEM(en, en),
            ITEM(an, an),
            ITEM(eng, eg),
            ITEM(ang, ah)
        };
        static std::unordered_map<StringView, StringView> initial_map{
            ITEM(zh, v), ITEM(ch, i), ITEM(sh, u),
        };
        static std::unordered_map<StringView, StringView> final_map{
            ITEM(i, i), ITEM(u, u), ITEM(v, v),
            ITEM(e, e), ITEM(ie, x), ITEM(o, o), ITEM(uo, o), ITEM(ue, t), ITEM(ve, t),
            ITEM(a, a), ITEM(ia, w), ITEM(ua, w),
            ITEM(ei, z), ITEM(ui, v),
            ITEM(ai, l), ITEM(uai, y),
            ITEM(ou, b), ITEM(iu, q),
            ITEM(ao, k), ITEM(iao, c),
            ITEM(in, n), ITEM(un, p), ITEM(vn, p),
            ITEM(en, f),
            ITEM(an, j), ITEM(ian, m), ITEM(uan, r), ITEM(van, r),
            ITEM(ing, ;),
            ITEM(ong, s), ITEM(iong, s),
            ITEM(eng, g),
            ITEM(ang, h), ITEM(iang, d), ITEM(uang, d),
            ITEM(er, er)
        };
#undef ITEM
        return convert(pinyin_map, initial_map, final_map);
    }
}