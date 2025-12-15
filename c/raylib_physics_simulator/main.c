#include <raylib.h>
#include <raymath.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

struct Ball {
    Vector2 position;
    Vector2 velocity;
    float mass;
    float radius;
};

int main(void) {
    srand(time(NULL));

    const int screenWidth = 1000;
    const int screenHeight = 800;
    const char *title = "Hello, Raylib";

    InitWindow(screenWidth, screenHeight, title);

    SetTargetFPS(60);

    Vector2 center = {.x = (float)screenWidth / 2,
                      .y = (float)screenHeight / 2};

    const int pos_rand_range = 400;
    const int vel_rand_range = 00;
    const float mass_rand_start = 0.2;
    const float mass_rand_end = 0.2;
    const float inv_density = 40;

#define BALL_COUNT 2000
    struct Ball balls[BALL_COUNT];
    for (int i = 0; i < BALL_COUNT; i++) {
        balls[i].mass = mass_rand_start + ((float)rand() / (float)RAND_MAX *
                                           (mass_rand_end - mass_rand_start));
        balls[i].radius = balls[i].mass * inv_density;
        balls[i].position.x =
            center.x + ((float)rand() / (float)RAND_MAX * 2 * pos_rand_range) -
            pos_rand_range;
        balls[i].position.y =
            center.y + ((float)rand() / (float)RAND_MAX * 2 * pos_rand_range) -
            pos_rand_range;
        balls[i].velocity.x =
            ((float)rand() / (float)RAND_MAX * 2 * vel_rand_range) -
            vel_rand_range;
        balls[i].velocity.y =
            ((float)rand() / (float)RAND_MAX * 2 * vel_rand_range) -
            vel_rand_range;
    }

    while (!WindowShouldClose()) {
        BeginDrawing();

        ClearBackground(RAYWHITE);

        // Update

        // Forces
        for (int i = 0; i < BALL_COUNT; i++) {
            Vector2 *position = &balls[i].position;
            Vector2 *velocity = &balls[i].velocity;

			Vector2 delta_pos = Vector2Subtract(center, balls[i].position);
			Vector2 delta_pos_n = Vector2Normalize(delta_pos);
            Vector2 gravity = Vector2Scale(delta_pos_n, 100);

            *velocity =
                Vector2Add(*velocity, Vector2Scale(gravity, GetFrameTime() /
                                                                balls[i].mass));
            DrawCircleV(*position, balls[i].radius, BLUE);
        }

        // Collisions
        for (int i = 0; i < BALL_COUNT; i++) {
            for (int j = i + 1; j < BALL_COUNT; j++) {
                float min_dist = balls[i].radius + balls[j].radius;
                float min_dist_sq = min_dist * min_dist;

                // momentum math
                if (Vector2DistanceSqr(
                        Vector2Add(
                            balls[i].position,
                            Vector2Scale(balls[i].velocity, GetFrameTime())),
                        Vector2Add(
                            balls[j].position,
                            Vector2Scale(balls[j].velocity, GetFrameTime()))) <=
                    min_dist_sq) {
                    Vector2 delta_pos =
                        Vector2Subtract(balls[j].position, balls[i].position);
                    Vector2 normal =
                        Vector2Normalize(delta_pos); // Axis of collision

                    Vector2 projected_v1 = Vector2Scale(
                        normal, Vector2DotProduct(balls[i].velocity, normal));
                    Vector2 projected_v2 =
                        Vector2Scale(Vector2Negate(normal),
                                     Vector2DotProduct(balls[j].velocity,
                                                       Vector2Negate(normal)));

                    Vector2 new_projected_v1 = Vector2Scale(
                        Vector2Add(
                            Vector2Scale(projected_v2, 2 * balls[j].mass),
                            Vector2Scale(projected_v1,
                                         balls[j].mass - balls[i].mass)),
                        1 / (balls[i].mass + balls[j].mass));
                    Vector2 new_projected_v2 = Vector2Scale(
                        Vector2Add(
                            Vector2Scale(projected_v1, 2 * balls[i].mass),
                            Vector2Scale(projected_v2,
                                         balls[i].mass - balls[j].mass)),
                        1 / (balls[i].mass + balls[j].mass));
                    new_projected_v1 = Vector2Scale(new_projected_v1, 0.9);
                    new_projected_v2 = Vector2Scale(new_projected_v2, 0.9);

                    balls[i].velocity = Vector2Add(
                        Vector2Subtract(balls[i].velocity, projected_v1),
                        new_projected_v1);
                    balls[j].velocity = Vector2Add(
                        Vector2Subtract(balls[j].velocity, projected_v2),
                        new_projected_v2);
                }
            }
        }
        for (int i = 0; i < BALL_COUNT; i++) {
            Vector2 nudge = {0, 0};
            for (int j = 0; j < BALL_COUNT; j++) {
                float min_dist = balls[i].radius + balls[j].radius;
                float min_dist_sq = min_dist * min_dist;
                // Nudge off other balls
                if (Vector2DistanceSqr(balls[i].position, balls[j].position) <=
                    min_dist_sq) {
                    Vector2 delta_pos =
                        Vector2Subtract(balls[j].position, balls[i].position);
                    Vector2 normal =

                        Vector2Normalize(delta_pos); // Axis of collision
                    Vector2 nudge_1 = Vector2Scale(
                        normal, -(min_dist - Vector2Length(delta_pos)));

                    nudge = Vector2Add(nudge, nudge_1);
                }
            }
            balls[i].position =
                Vector2Add(balls[i].position, Vector2Scale(nudge, 0.5f));
        }

        // Update position
        for (int i = 0; i < sizeof(balls) / sizeof(struct Ball); i++) {
            balls[i].position =
                Vector2Add(balls[i].position,
                           Vector2Scale(balls[i].velocity, GetFrameTime()));

            // Wall collisions
            if (balls[i].position.x + balls[i].radius >= GetScreenWidth()) {
                balls[i].velocity.x = -fabsf(balls[i].velocity.x);
            }
            if (balls[i].position.x - balls[i].radius <= 0) {
                balls[i].velocity.x = fabsf(balls[i].velocity.x);
            }
            if (balls[i].position.y + balls[i].radius >= GetScreenHeight()) {
                balls[i].velocity.y = -fabsf(balls[i].velocity.y);
            }
            if (balls[i].position.y - balls[i].radius <= 0) {
                balls[i].velocity.y = fabsf(balls[i].velocity.y);
            }
            if (balls[i].position.x + balls[i].radius >= GetScreenWidth()) {
                balls[i].position.x = GetScreenWidth() - balls[i].radius;
            }
            if (balls[i].position.x - balls[i].radius <= 0) {
                balls[i].position.x = balls[i].radius;
            }
            if (balls[i].position.y + balls[i].radius >= GetScreenHeight()) {
                balls[i].position.y = GetScreenHeight() - balls[i].radius;
            }
            if (balls[i].position.y - balls[i].radius <= 0) {
                balls[i].position.y = balls[i].radius;
            }

            DrawCircleV(balls[i].position, balls[i].radius, BLUE);
        }

        EndDrawing();

        printf("%d FPS\n", GetFPS());
    }

    return 0;
}
