# The app ID to use.
#
# Use de.swsnr.umbrella for the standard app ID, and de.swsnr.umbrella.Devel to
# build a nightly snapshot.  Other values are not supported.
APPID = de.swsnr.umbrella

BLUEPRINTS = $(wildcard ui/*.blp)
CATALOGS = $(wildcard po/*.po)

XGETTEXT_OPTS = \
	--package-name=$(APPID) \
	--foreign-user --copyright-holder "Sebastian Wiesner <sebastian@swsnr.de>" \
	--sort-by-file --from-code=UTF-8 --add-comments

# Extract the message template from all source files.
#
# You typically do not need to run this manually: The gettext Github workflow
# watches for changes to relevant source files, runs this target, and opens a
# pull request with the corresponding changes.
#
# When changing the set of files taken into account for xgettext also update the
# paths list in the gettext.yml workflow to make sure that updates to these
# files are caught by the gettext workflows.
#
# We strip the POT-Creation-Date from the resulting POT because xgettext bumps
# it everytime regardless if anything else changed, and this just generates
# needless diffs.
.PHONY: pot
pot:
	find src -name '*.rs' > po/POTFILES.rs
	find resources/ -name '*.blp' > po/POTFILES.blp
	xgettext $(XGETTEXT_OPTS) --language=C --keyword=dpgettext2:2c,3 --files-from=po/POTFILES.rs --output=po/de.swsnr.umbrella.rs.pot
	xgettext $(XGETTEXT_OPTS) --language=C --keyword=_ --keyword=C_:1c,2 --files-from=po/POTFILES.blp --output=po/de.swsnr.umbrella.blp.pot
	xgettext $(XGETTEXT_OPTS) --output=po/de.swsnr.umbrella.pot \
		po/de.swsnr.umbrella.rs.pot po/de.swsnr.umbrella.blp.pot \
		resources/de.swsnr.umbrella.metainfo.xml.in
	rm -f po/POTFILES* po/de.swsnr.umbrella.rs.pot po/de.swsnr.umbrella.blp.pot
	sed -i /POT-Creation-Date/d po/de.swsnr.umbrella.pot
