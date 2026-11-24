// SPDX-License-Identifier: GPL-2.0-only

#include <linux/backlight.h>
#include <linux/gpio/consumer.h>
#include <linux/i2c.h>
#include <linux/module.h>
#include <linux/of.h>
#include <linux/property.h>
#include <linux/regulator/consumer.h>
#include <linux/slab.h>

extern u16 ktz8866_rust_max_brightness(void);
extern int ktz8866_rust_sink_mask(u32 sinks);
extern int ktz8866_rust_set_brightness(const struct i2c_client *client,
				     u8 mask, u16 brightness);

struct ktz8866_backlight {
	struct i2c_client *client;
	u8 sink_mask;
};

static int ktz8866_update_status(struct backlight_device *backlight)
{
	struct ktz8866_backlight *data = bl_get_data(backlight);

	return ktz8866_rust_set_brightness(data->client, data->sink_mask,
					  backlight_get_brightness(backlight));
}

static const struct backlight_ops ktz8866_ops = {
	.options = BL_CORE_SUSPENDRESUME,
	.update_status = ktz8866_update_status,
};

static int ktz8866_probe(struct i2c_client *client)
{
	const struct backlight_properties properties = {
		.type = BACKLIGHT_RAW,
		.max_brightness = ktz8866_rust_max_brightness(),
		.scale = BACKLIGHT_SCALE_LINEAR,
	};
	struct device *dev = &client->dev;
	struct ktz8866_backlight *data;
	struct backlight_device *backlight;
	struct gpio_desc *enable;
	u32 sinks = 6;
	int ret;

	if (!i2c_check_functionality(client->adapter, I2C_FUNC_SMBUS_WRITE_BYTE_DATA))
		return -EOPNOTSUPP;

	data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
	if (!data)
		return -ENOMEM;
	data->client = client;
	if (device_property_present(dev, "current-num-sinks")) {
		ret = device_property_read_u32(dev, "current-num-sinks", &sinks);
		if (ret)
			return dev_err_probe(dev, ret, "invalid current-num-sinks\n");
	}
	ret = ktz8866_rust_sink_mask(sinks);
	if (ret < 0)
		return dev_err_probe(dev, ret, "current-num-sinks must be 1 to 6\n");
	data->sink_mask = ret;

	ret = devm_regulator_get_enable(dev, "vddpos");
	if (ret)
		return dev_err_probe(dev, ret, "could not enable vddpos\n");
	ret = devm_regulator_get_enable(dev, "vddneg");
	if (ret)
		return dev_err_probe(dev, ret, "could not enable vddneg\n");

	enable = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_HIGH);
	if (IS_ERR(enable))
		return dev_err_probe(dev, PTR_ERR(enable), "could not enable backlight\n");

	ret = ktz8866_rust_set_brightness(client, data->sink_mask, 0);
	if (ret)
		return dev_err_probe(dev, ret, "could not blank backlight\n");

	backlight = devm_backlight_device_register(dev, dev_name(dev), dev,
						  data, &ktz8866_ops, &properties);
	if (IS_ERR(backlight))
		return PTR_ERR(backlight);

	i2c_set_clientdata(client, backlight);
	return 0;
}

static void ktz8866_shutdown(struct i2c_client *client)
{
	struct backlight_device *backlight = i2c_get_clientdata(client);

	backlight_disable(backlight);
}

static const struct of_device_id ktz8866_of_match[] = {
	{ .compatible = "kinetic,ktz8866" },
	{ }
};
MODULE_DEVICE_TABLE(of, ktz8866_of_match);

static struct i2c_driver ktz8866_driver = {
	.driver = {
		.name = "ktz8866-rust",
		.of_match_table = ktz8866_of_match,
	},
	.probe = ktz8866_probe,
	.remove = ktz8866_shutdown,
	.shutdown = ktz8866_shutdown,
};
module_i2c_driver(ktz8866_driver);

MODULE_DESCRIPTION("Kinetic KTZ8866 Rust backlight driver");
MODULE_LICENSE("GPL v2");
