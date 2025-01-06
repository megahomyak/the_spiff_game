#include <stdio.h>

/* Disclaimer: I'm not using NULL here because I think it sucks */
/* "Game engine code" a.k.a. "game library code": */

struct item {
    char* sprite;
};

struct world_ctx { /* State of game world. Stored in permanent memory */
    struct location* location;
    struct {
        int exists;
        struct item* item;
    } spooned_item;
};

struct ctx { /* Context with runtime trash (=runtime necessities) included */
    /* Canvas handle here, audio handle here */
    int world_changed;
    struct world_ctx world;
};

struct button_handler {
    int exists;
    void (*function)(struct ctx*);
};

struct location {
    void (*render_handler)(struct ctx*); /* Mustn't edit the context, must only render */
    /* Any below must not render anything, but may edit the context (even switch the location, yes, that as well) */
    struct button_handler spin_handler;
    struct button_handler spoon_handler;
    struct button_handler spank_handler;
    struct button_handler sprint_handler;
    struct button_handler spend_handler;
    struct button_handler speedrun_handler;
};

typedef double unit_interval;

void draw(struct ctx* ctx, char* image_path, unit_interval lowleft, unit_interval lowright, unit_interval upleft, unit_interval upright) {
    printf("Drawing %s at (%lf, %lf), (%lf, %lf)\n", image_path, lowleft, lowright, upleft, upright);
}

void play(struct ctx* ctx, char* sound_path) {
    printf("Playing %s\n", sound_path);
}

void render_spoon(struct ctx* ctx) {
    if (ctx->world.spooned_item.exists) {
        draw(ctx, "spoon.png", 0.9, 0.9, 1.0, 1.0);
        draw(ctx, ctx->world.spooned_item.item->sprite, 0.92, 0.92, 0.98, 0.98);
    }
}

void switch_location(struct ctx* ctx, struct location* new_location) {
    ctx->world_changed = 1;
    ctx->world.location = new_location;
}

void render_world(struct ctx* ctx) {
    printf("Clearing the board before drawing anything... (Let's say we're drawing with SVGs)\n");
    ctx->world.location->render_handler(ctx);
}

void handle_button_press(struct ctx* ctx, struct button_handler* handler) {
    if (handler->exists) {
        handler->function(ctx);
        if (ctx->world_changed) {
            render_world(ctx);
        }
    }
}

/* "User code": */

void render_first_location(struct ctx* ctx) { /* This is an "awake" location */
    draw(ctx, "first_location.png", 0.0, 0.0, 1.0, 1.0);
    render_spoon(ctx);
}
struct location SECOND_LOCATION;
void first_location_spin(struct ctx* ctx) {
    play(ctx, "spin.oga");
    switch_location(ctx, &SECOND_LOCATION);
}

void render_second_location(struct ctx* ctx) { /* This is a "nightmare" location */
    draw(ctx, "second_location.png", 0.0, 0.0, 1.0, 1.0);
}

#define NO_HANDLER { .exists = 0 }
#define YES_HANDLER(handler) { .exists = 1, .function = handler }
struct location FIRST_LOCATION = {
    .spin_handler = YES_HANDLER(first_location_spin),
    .spoon_handler = NO_HANDLER,
    .spank_handler = NO_HANDLER,
    .sprint_handler = NO_HANDLER,
    .spend_handler = NO_HANDLER,
    .speedrun_handler = NO_HANDLER,
    .render_handler = render_first_location,
};
struct location SECOND_LOCATION = {
    .spin_handler = NO_HANDLER,
    .spoon_handler = NO_HANDLER,
    .spank_handler = NO_HANDLER,
    .sprint_handler = NO_HANDLER,
    .spend_handler = NO_HANDLER,
    .speedrun_handler = NO_HANDLER,
    .render_handler = render_second_location,
};

int main(void) {
    struct ctx ctx = {
        .world_changed = 0,
        .world = {
            .location = &FIRST_LOCATION,
        },
    };
    render_world(&ctx); /* The initial render, very important... */

    /* Simulation of player behavior (since the player can only press buttons:) */
    printf("Trying to \"Spank\"\n");
    handle_button_press(&ctx, &ctx.world.location->spank_handler); /* Nothing should happen */
    printf("Trying to \"Spin\"\n");
    handle_button_press(&ctx, &ctx.world.location->spin_handler); /* The spin handler should be invoked */
    printf("Trying to \"Spin\"\n");
    handle_button_press(&ctx, &ctx.world.location->spin_handler); /* Nothing should happen */
}
