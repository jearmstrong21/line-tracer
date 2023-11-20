#include geom/mod

float sampleWavelength(RAND_PARAM, float t) {
//    return RAND_FLOAT * 300.0 + 400.0;
    return t * 300.0 + 400.0;
}

uniform vec2 light_origin;
uniform float light_angle;

EdgeSample sampleLightsSpaced(RAND_PARAM, float t) {
//    vec2 n = vec2(1, 0);
//    vec2 n = normalize(vec2(1.0, 0.1));
//    if (RAND_FLOAT < -0.5) {
//        n = RAND_HEMI(n);
//    }
    /*
    AIM AT (0.5,0.5)
    start from (0.3,0.1)
    */
//    vec2 origin = screenSize*vec2(0.2,0.2);
    vec2 origin = light_origin;
    vec2 target = origin + vec2(cos(light_angle), sin(light_angle));
    vec2 direction=normalize(target-origin);
    vec2 left=vec2(-direction.y,direction.x);
    return EdgeSample(origin + (t-0.5)*left * 5.0, direction);
//    return EdgeSample(vec2(0.0, cos(systemTime) * 0.0 + screenSize.y * 0.5 + (t - 0.5) * 5.0), n);
}

uniform float eta_a;
uniform float eta_b;

float getEta(float wl) {
//    return 12.5;
    wl /= 700.0;
    return eta_a + eta_b / (wl * wl);
//    return 0.9 + 0.4 / (wl * wl);
//    return 1.0 / 1.0 * 400.0 / wl;
}

//#define H_SCENE_EPS 1E10
//HitRes hitScene(Ray r) {
//    HitRes hr;
//    hr.t = H_SCENE_EPS;
//
//    for(int i=0;i<)
//
////    hitCircle(r, Circle(vec2(600.0, 400.0 + 50.0 * cos(0.5 * systemTime)), 150.0, M_REFR), hr);
//    hitCircle(r, Circle(mouseCoords, 150.0, M_REFR), hr);
//
//    hitLine(r, Line(vec2(800.0, 300.0), vec2(1000.0, 300.0), M_REFR), hr);
//    hitLine(r, Line(vec2(900.0, 500.0), vec2(1000.0, 300.0), M_REFR), hr);
//    hitLine(r, Line(vec2(900.0, 500.0), vec2(800.0, 300.0), M_REFR), hr);
//
//    hitArc(r, Arc(vec2(1400.0, 400.0), 250.0, -120.0, 120.0, M_REFL), hr);
//
//    hitLine(r, Line(vec2(0, 0), vec2(screenSize.x, 0), M_REFL), hr);
//    hitLine(r, Line(vec2(0, screenSize.y), vec2(screenSize), M_REFL), hr);
////    hitLine(r, Line(vec2(0, 0), vec2(0, screenSize.y), M_REFL), hr);
////    hitLine(r, Line(vec2(0, screenSize.y), vec2(mouseCoords), M_REFL), hr);
////    hitLine(r, Line(vec2(screenSize.x, 0), screenSize), hr);
////    hitLine(r, Line(screenSize, vec2(0, screenSize.y)), hr);
////    hitLine(r, Line(vec2(0, screenSize.y), vec2(0, 0)), hr);
//
//    hr.hit = hr.t < H_SCENE_EPS;
//    if (hr.hit) {
//        hr.n = normalize(hr.n);
//        if (dot(hr.n, r.d) > 0.0) {
//            hr.n *= -1.0;
//        }
//    }
//    return hr;
//}